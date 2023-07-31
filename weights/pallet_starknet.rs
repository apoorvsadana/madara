
//! Autogenerated weights for pallet_starknet
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `matthiass-mbp.home`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/madara
// benchmark
// pallet
// --execution=native
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 16
// --repeat
// 1
// --template=./scripts/benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// ./weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_starknet.
pub trait WeightInfo {
	fn infinite_loop() -> Weight;
}

/// Weights for pallet_starknet using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Starknet ContractClassHashes (r:2 w:0)
	/// Proof: Starknet ContractClassHashes (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Starknet FeeTokenAddress (r:1 w:0)
	/// Proof: Starknet FeeTokenAddress (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Starknet SequencerAddress (r:1 w:0)
	/// Proof: Starknet SequencerAddress (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Starknet Nonces (r:1 w:0)
	/// Proof: Starknet Nonces (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Starknet ContractClasses (r:2 w:0)
	/// Proof: Starknet ContractClasses (max_values: None, max_size: Some(20971552), added: 20974027, mode: MaxEncodedLen)
	fn infinite_loop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `34008`
		//  Estimated: `41949044`
		// Minimum execution time: 7_970_955_000_000 picoseconds.
		Weight::from_parts(7_970_955_000_000, 41949044)
			.saturating_add(T::DbWeight::get().reads(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Starknet ContractClassHashes (r:2 w:0)
	/// Proof: Starknet ContractClassHashes (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Starknet FeeTokenAddress (r:1 w:0)
	/// Proof: Starknet FeeTokenAddress (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Starknet SequencerAddress (r:1 w:0)
	/// Proof: Starknet SequencerAddress (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Starknet Nonces (r:1 w:0)
	/// Proof: Starknet Nonces (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Starknet ContractClasses (r:2 w:0)
	/// Proof: Starknet ContractClasses (max_values: None, max_size: Some(20971552), added: 20974027, mode: MaxEncodedLen)
	fn infinite_loop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `34008`
		//  Estimated: `41949044`
		// Minimum execution time: 7_970_955_000_000 picoseconds.
		Weight::from_parts(7_970_955_000_000, 41949044)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
	}
}
