use std::sync::Arc;

use starknet_accounts::{Account, Call, SingleOwnerAccount};
use starknet_core::chain_id;
use starknet_core::types::contract::legacy::LegacyContractClass;
use starknet_core::types::contract::{CompiledClass, SierraClass};
use starknet_core::types::{
    BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs, DeclareTransaction, FieldElement, FunctionCall,
    InvokeTransaction, Transaction,
};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_providers::Provider;
use starknet_signers::{LocalWallet, SigningKey};

use crate::constants::{FEE_TOKEN_ADDRESS, MAX_FEE_OVERRIDE};
use crate::{RpcAccount, TransactionDeclaration, TransactionExecution, TransactionLegacyDeclaration};

pub fn create_account<'a>(
    rpc: &'a JsonRpcClient<HttpTransport>,
    private_key: &str,
    account_address: &str,
    is_legacy: bool,
) -> RpcAccount<'a> {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(FieldElement::from_hex_be(private_key).unwrap()));
    let account_address = FieldElement::from_hex_be(account_address).expect("Invalid Contract Address");
    let execution_encoding = if is_legacy {
        starknet_accounts::ExecutionEncoding::Legacy
    } else {
        starknet_accounts::ExecutionEncoding::New
    };
    SingleOwnerAccount::new(rpc, signer, account_address, chain_id::TESTNET, execution_encoding)
}

pub async fn read_erc20_balance<'a>(
    rpc: &'a JsonRpcClient<HttpTransport>,
    contract_address: FieldElement,
    account_address: FieldElement,
) -> Vec<FieldElement> {
    rpc.call(
        FunctionCall {
            contract_address,
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![account_address],
        },
        BlockId::Tag(BlockTag::Latest),
    )
    .await
    .unwrap()
}

pub trait AccountActions {
    fn transfer_tokens(
        &self,
        recipient: FieldElement,
        transfer_amount: FieldElement,
        nonce: Option<u64>,
    ) -> TransactionExecution;

    fn declare_contract(
        &self,
        path_to_sierra: &str,
        path_to_casm: &str,
    ) -> (TransactionDeclaration, FieldElement, FieldElement);

    fn declare_legacy_contract(&self, path_to_compiled_contract: &str) -> (TransactionLegacyDeclaration, FieldElement);
}

impl AccountActions for SingleOwnerAccount<&JsonRpcClient<HttpTransport>, LocalWallet> {
    fn transfer_tokens(
        &self,
        recipient: FieldElement,
        transfer_amount: FieldElement,
        nonce: Option<u64>,
    ) -> TransactionExecution {
        let fee_token_address = FieldElement::from_hex_be(FEE_TOKEN_ADDRESS).unwrap();

        let calls = vec![Call {
            to: fee_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![recipient, transfer_amount, FieldElement::ZERO],
        }];

        // starknet-rs calls estimateFee with incorrect version which throws an error
        let max_fee = FieldElement::from_hex_be(MAX_FEE_OVERRIDE).unwrap();

        // TODO: add support for nonce with raw execution e.g https://github.com/0xSpaceShard/starknet-devnet-rs/blob/main/crates/starknet/src/starknet/add_invoke_transaction.rs#L10
        match nonce {
            Some(_nonce) => self.execute(calls).max_fee(max_fee),
            None => self.execute(calls).max_fee(max_fee),
        }
    }

    fn declare_contract(
        &self,
        path_to_sierra: &str,
        path_to_casm: &str,
    ) -> (TransactionDeclaration, FieldElement, FieldElement) {
        let sierra: SierraClass = serde_json::from_reader(
            std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path_to_sierra).unwrap(),
        )
        .unwrap();
        let casm: CompiledClass = serde_json::from_reader(
            std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path_to_casm).unwrap(),
        )
        .unwrap();
        let compiled_class_hash = casm.class_hash().unwrap();
        (
            self.declare(sierra.clone().flatten().unwrap().into(), compiled_class_hash)
				// starknet-rs calls estimateFee with incorrect version which throws an error
                .max_fee(FieldElement::from_hex_be(MAX_FEE_OVERRIDE).unwrap()),
            sierra.class_hash().unwrap(),
            compiled_class_hash,
        )
    }

    fn declare_legacy_contract(&self, path_to_compiled_contract: &str) -> (TransactionLegacyDeclaration, FieldElement) {
        let contract_artifact: LegacyContractClass = serde_json::from_reader(
            std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + path_to_compiled_contract).unwrap(),
        )
        .unwrap();
        (
            self.declare_legacy(Arc::new(contract_artifact.clone()))
			 // starknet-rs calls estimateFee with incorrect version which throws an error
			 .max_fee(FieldElement::from_hex_be(MAX_FEE_OVERRIDE).unwrap()),
            contract_artifact.class_hash().unwrap(),
        )
    }
}

// a short way to do it is to serialize both blocks and compare them
// however, in case of failures, the assert messages will be less informative
// hence, we compare each field separately
pub fn assert_equal_blocks_with_tx_hashes(b1: BlockWithTxHashes, b2: BlockWithTxHashes) {
    assert_eq!(b1.transactions, b2.transactions);
    assert_eq!(b1.status, b1.status);
    assert_eq!(b1.block_hash, b2.block_hash);
    assert_eq!(b1.parent_hash, b2.parent_hash);
    assert_eq!(b1.block_number, b2.block_number);
    assert_eq!(b1.new_root, b2.new_root);
    assert_eq!(b1.sequencer_address, b2.sequencer_address);
}

pub fn assert_equal_blocks_with_txs(b1: BlockWithTxs, b2: BlockWithTxs) {
    assert_eq!(b1.status, b1.status);
    assert_eq!(b1.block_hash, b2.block_hash);
    assert_eq!(b1.parent_hash, b2.parent_hash);
    assert_eq!(b1.block_number, b2.block_number);
    assert_eq!(b1.new_root, b2.new_root);
    assert_eq!(b1.sequencer_address, b2.sequencer_address);
    assert_eq!(b1.transactions.len(), b2.transactions.len());
    for (tx1, tx2) in b1.transactions.iter().zip(b2.transactions.iter()) {
        assert_equal_transactions(tx1, tx2);
    }
}

pub fn assert_equal_transactions(tx1: &Transaction, tx2: &Transaction) {
    match tx1 {
        Transaction::Invoke(InvokeTransaction::V1(tx1)) => {
            let tx2 = match tx2 {
                Transaction::Invoke(InvokeTransaction::V1(tx)) => tx,
                _ => panic!("Expected Invoke transaction"),
            };
            assert_eq!(tx1.transaction_hash, tx2.transaction_hash);
            assert_eq!(tx1.max_fee, tx2.max_fee);
            assert_eq!(tx1.signature, tx2.signature);
            assert_eq!(tx1.nonce, tx2.nonce);
            assert_eq!(tx1.sender_address, tx2.sender_address);
            assert_eq!(tx1.calldata, tx2.calldata);
        }
        Transaction::L1Handler(tx1) => {
            let tx2 = match tx2 {
                Transaction::L1Handler(tx) => tx,
                _ => panic!("Expected L1Handler transaction"),
            };
            assert_eq!(tx1.transaction_hash, tx2.transaction_hash);
            assert_eq!(tx1.version, tx2.version);
            assert_eq!(tx1.nonce, tx2.nonce);
            assert_eq!(tx1.contract_address, tx2.contract_address);
            assert_eq!(tx1.entry_point_selector, tx2.entry_point_selector);
            assert_eq!(tx1.calldata, tx2.calldata);
        }
        Transaction::Declare(DeclareTransaction::V2(tx1)) => {
            let tx2 = match tx2 {
                Transaction::Declare(DeclareTransaction::V2(tx)) => tx,
                _ => panic!("Expected DeclareV2 transaction"),
            };
            assert_eq!(tx1.nonce, tx2.nonce);
            assert_eq!(tx1.sender_address, tx2.sender_address);
            assert_eq!(tx1.max_fee, tx2.max_fee);
            assert_eq!(tx1.signature, tx2.signature);
            assert_eq!(tx1.class_hash, tx2.class_hash);
            assert_eq!(tx1.compiled_class_hash, tx2.compiled_class_hash);
            assert_eq!(tx1.transaction_hash, tx2.transaction_hash);
        }
        Transaction::Declare(DeclareTransaction::V1(tx1)) => {
            let tx2 = match tx2 {
                Transaction::Declare(DeclareTransaction::V1(tx)) => tx,
                _ => panic!("Expected DeclareV1 transaction"),
            };
            assert_eq!(tx1.nonce, tx2.nonce);
            assert_eq!(tx1.sender_address, tx2.sender_address);
            assert_eq!(tx1.max_fee, tx2.max_fee);
            assert_eq!(tx1.signature, tx2.signature);
            assert_eq!(tx1.class_hash, tx2.class_hash);
            assert_eq!(tx1.transaction_hash, tx2.transaction_hash);
        }
        Transaction::DeployAccount(tx1) => {
            let tx2 = match tx2 {
                Transaction::DeployAccount(tx) => tx,
                _ => panic!("Expected DeployAccount transaction"),
            };
            assert_eq!(tx1.transaction_hash, tx2.transaction_hash);
            assert_eq!(tx1.max_fee, tx2.max_fee);
            assert_eq!(tx1.signature, tx2.signature);
            assert_eq!(tx1.nonce, tx2.nonce);
            assert_eq!(tx1.contract_address_salt, tx2.contract_address_salt);
            assert_eq!(tx1.constructor_calldata, tx2.constructor_calldata);
            assert_eq!(tx1.class_hash, tx2.class_hash);
        }
        _ => unimplemented!("transaction either deprecated or will be deprecated in the future"),
    }
}