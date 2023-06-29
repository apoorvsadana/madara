use blockifier::execution::contract_class::EntryPointV1;
use blockifier::execution::errors::EntryPointExecutionError;
use serde::{Deserialize, Serialize};
use sp_core::ConstU32;
use starknet_api::api_core::EntryPointSelector;
use starknet_api::deprecated_contract_class::{EntryPoint, EntryPointOffset, EntryPointType};
use starknet_api::hash::StarkFelt;
use starknet_api::StarknetApiError;
use starknet_ff::{FieldElement, FromByteArrayError};
use thiserror_no_std::Error;

use crate::scale_codec::{Decode, Encode, Error, Input, MaxEncodedLen, Output};
/// Max number of entrypoints.
pub type MaxEntryPoints = ConstU32<4294967295>;

/// Wrapper type for transaction execution result.
pub type EntryPointExecutionResultWrapper<T> = Result<T, EntryPointExecutionErrorWrapper>;

/// Enum that represents all the entrypoints types.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Encode,
    Decode,
    scale_info::TypeInfo,
    MaxEncodedLen,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum EntryPointTypeWrapper {
    /// A constructor entry point.
    #[serde(rename = "CONSTRUCTOR")]
    Constructor,
    /// An external entry point.
    #[serde(rename = "EXTERNAL")]
    #[default]
    External,
    /// An L1 handler entry point.
    #[serde(rename = "L1_HANDLER")]
    L1Handler,
}

// Traits implementation.
impl From<EntryPointType> for EntryPointTypeWrapper {
    fn from(entry_point_type: EntryPointType) -> Self {
        match entry_point_type {
            EntryPointType::Constructor => EntryPointTypeWrapper::Constructor,
            EntryPointType::External => EntryPointTypeWrapper::External,
            EntryPointType::L1Handler => EntryPointTypeWrapper::L1Handler,
        }
    }
}

impl From<EntryPointTypeWrapper> for EntryPointType {
    fn from(entrypoint: EntryPointTypeWrapper) -> Self {
        match entrypoint {
            EntryPointTypeWrapper::Constructor => EntryPointType::Constructor,
            EntryPointTypeWrapper::External => EntryPointType::External,
            EntryPointTypeWrapper::L1Handler => EntryPointType::L1Handler,
        }
    }
}

/// Representation of a EntryPoint used in ContractClassV0Inner
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntryPointV0Wrapper(EntryPoint);
/// SCALE trait.
impl Encode for EntryPointV0Wrapper {
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.0.selector.0.0);
        dest.write(&self.0.offset.0.to_be_bytes());
    }
}
/// SCALE trait.
impl Decode for EntryPointV0Wrapper {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        let mut selector = [0u8; 32];
        // Use this because usize can be of different byte size.
        let mut offset = [0u8; core::mem::size_of::<usize>()];
        input.read(&mut selector)?;
        input.read(&mut offset)?;

        Ok(EntryPointV0Wrapper(EntryPoint {
            selector: EntryPointSelector(StarkFelt(selector)),
            offset: EntryPointOffset(usize::from_be_bytes(offset)),
        }))
    }
}

// Traits implementation.

impl From<EntryPoint> for EntryPointV0Wrapper {
    fn from(entry_point: EntryPoint) -> Self {
        Self(entry_point)
    }
}

impl From<EntryPointV0Wrapper> for EntryPoint {
    fn from(entry_point: EntryPointV0Wrapper) -> Self {
        entry_point.0
    }
}

/// Representation of a EntryPoint used in ContractClassV1Inner
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryPointV1Wrapper(EntryPointV1);
/// SCALE trait.
impl Encode for EntryPointV1Wrapper {
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.0.selector.0.0);
        dest.write(&self.0.offset.0.to_be_bytes());
        dest.write(&Encode::encode(&self.0.builtins));
    }
}
/// SCALE trait.
impl Decode for EntryPointV1Wrapper {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        let mut selector = [0u8; 32];
        // Use this because usize can be of different byte size.
        let mut offset = [0u8; core::mem::size_of::<usize>()];
        input.read(&mut selector)?;
        input.read(&mut offset)?;

        Ok(EntryPointV1Wrapper(EntryPointV1 {
            selector: EntryPointSelector(StarkFelt(selector)),
            offset: EntryPointOffset(usize::from_be_bytes(offset)),
            builtins: Decode::decode(input)?, // decoding the remaining input as a vector of strings
        }))
    }
}

// Traits implementation.

impl From<EntryPointV1> for EntryPointV1Wrapper {
    fn from(entry_point: EntryPointV1) -> Self {
        Self(entry_point)
    }
}

impl From<EntryPointV1Wrapper> for EntryPointV1 {
    fn from(entry_point: EntryPointV1Wrapper) -> Self {
        entry_point.0
    }
}

/// Wrapper type for transaction execution error.
#[derive(Debug, Error)]
pub enum EntryPointExecutionErrorWrapper {
    /// Transaction execution error.
    #[error(transparent)]
    EntryPointExecution(#[from] EntryPointExecutionError),
    /// Starknet API error.
    #[error(transparent)]
    StarknetApi(#[from] StarknetApiError),
    /// Block context serialization error.
    #[error("Block context serialization error")]
    BlockContextSerializationError,
}

#[cfg(feature = "std")]
mod reexport_std_types {
    use starknet_core::types::LegacyContractEntryPoint;

    use super::*;
    impl From<LegacyContractEntryPoint> for EntryPointV0Wrapper {
        fn from(value: LegacyContractEntryPoint) -> Self {
            let selector = EntryPointSelector(StarkFelt(value.selector.to_bytes_be()));
            let offset = EntryPointOffset(value.offset as usize);
            Self(EntryPoint { selector, offset })
        }
    }

    impl TryFrom<EntryPointV0Wrapper> for LegacyContractEntryPoint {
        type Error = FromByteArrayError;
        fn try_from(value: EntryPointV0Wrapper) -> Result<Self, Self::Error> {
            let selector = FieldElement::from_bytes_be(&value.0.selector.0.0)?;
            let offset = value.0.offset.0 as u64;
            Ok(Self { selector, offset })
        }
    }
}

#[cfg(feature = "std")]
pub use reexport_std_types::*;
