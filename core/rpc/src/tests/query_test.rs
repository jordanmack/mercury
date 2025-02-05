use super::*;

use crate::types::StructureType;
use tokio::test;

#[test]
async fn test_get_db_info() {
    let engine = RpcTestEngine::new().await;
    let rpc = engine.rpc(NetworkType::Testnet);
    let db_info = rpc.get_db_info().unwrap();
    println!("db info: {:?}", db_info);
    assert_eq!(db_info.db, DBDriver::PostgreSQL);
    assert_eq!(db_info.center_id, 0);
    assert_eq!(db_info.machine_id, 0);
    assert_eq!(db_info.conn_size, 100);
}

#[test]
async fn test_get_spent_transaction() {
    let engine = RpcTestEngine::new().await;
    let rpc = engine.rpc(NetworkType::Testnet);

    let outpoint = ckb_jsonrpc_types::OutPoint {
        tx_hash: h256!("0xb50ef2272f9f72b11e21ec12bd1b8fc9136cafc25c197b6fd4c2eb4b19fa905c"),
        index: 0u32.into(),
    };
    let payload = GetSpentTransactionPayload {
        outpoint,
        structure_type: StructureType::Native,
    };
    let res = rpc.get_spent_transaction(payload).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("10090"))
}

// fn query_test(
//     rpc: &MercuryRpcImpl<MemoryDB, CkbRpcClient>,
//     addr: &str,
//     expected_len: usize,
//     ret_index: usize,
//     expected_capacity: u64,
//     expect_amount: Option<u128>,
// ) {
//     let ret = rpc
//         .get_cells_by_lock_script(&(parse_address(addr).unwrap().payload().into()))
//         .unwrap();
//     assert_eq!(ret.len(), expected_len);

//     let capacity: u64 = ret[ret_index].0.cell_output.capacity().unpack();
//     assert_eq!(capacity, expected_capacity * BYTE_SHANNONS);

//     let data = ret[ret_index].0.cell_data.raw_data();
//     if let Some(amount) = expect_amount {
//         assert_eq!(amount.to_le_bytes().to_vec(), data);
//     } else {
//         assert!(data.is_empty());
//     }
// }

// #[test]
// fn test_get_cells_by_lock_script() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     let addr_3 = "ckt1qyq98qe26z8eg8q0852h622m40s50swtqnrqndruht";

//     let engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 500_000, 0, 0, 0),
//         AddressData::new(addr_2, 0, 200, 0, 0),
//         AddressData::new(addr_3, 600_000, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let expected_len = 1usize;
//     let ret_index = 0usize;

//     query_test(&rpc, addr_1, expected_len, ret_index, 500_000, None);
//     query_test(&rpc, addr_2, expected_len, ret_index, 142, Some(200));
//     query_test(&rpc, addr_3, expected_len, ret_index, 600_000, None);
// }

// #[test]
// fn test_get_ckb_balance() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     // let addr_3 = "ckt1qyq98qe26z8eg8q0852h622m40s50swtqnrqndruht";

//     let engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 500_000, 300, 0, 0),
//         AddressData::new(addr_2, 1000, 200, 100, 0),
//         // AddressData::new(addr_3, 600_000, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let ret_1 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     let ret_2 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_2.to_string()),
//             block_number: None,
//         })
//         .unwrap();

//     assert_eq!(
//         ret_1.balances[0].unconstrained,
//         (500142 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_2.balances[0].unconstrained,
//         (1142 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(ret_2.balances[0].locked, (142 * BYTE_SHANNONS).to_string());
// }

// #[test]
// #[ignore]
// fn test_get_ckb_balance_matured_cellbase() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     let mut engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 100_000, 400, 100, 0),
//         AddressData::new(addr_2, 100_000, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let ret_1_at_genesis = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     let ret_2_at_genesis = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_2.to_string()),
//             block_number: None,
//         })
//         .unwrap();

//     assert_eq!(
//         ret_1_at_genesis.balances[0].unconstrained,
//         (100_142 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_2_at_genesis.balances[0].unconstrained,
//         (100_000 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_1_at_genesis.balances[0].locked,
//         (142 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(ret_2_at_genesis.balances[0].locked, (0).to_string());

//     // Submit a cellbase tx mined by addr_1, expect to increase the locked balance by 1000 CKB
//     let cellbase_tx = RpcTestEngine::build_cellbase_tx(addr_1, 1000);
//     let block_1 = RpcTestEngine::new_block(vec![cellbase_tx], 1, 1);
//     engine.append(block_1);

//     assert_eq!(
//         ret_1_at_genesis.balances[0].unconstrained,
//         (100_142 * BYTE_SHANNONS).to_string()
//     );
//     let ret_at_block_1 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     assert_eq!(
//         ret_at_block_1.balances[0].locked,
//         (1142 * BYTE_SHANNONS).to_string()
//     );

//     // Submit another cellbase tx mined by addr_2, and set the block epoch bigger than `cellbase_maturity`,
//     // expect to:
//     // 1. increate addr_2's locked balance by 1000 CKB
//     // 2. increate addr_1's spendable balance by 1000 CKB, while reduce addr_1's locked balance by 1000 CKB
//     let cellbase_tx = RpcTestEngine::build_cellbase_tx(addr_2, 1000);
//     let block_2 = RpcTestEngine::new_block(vec![cellbase_tx], 2, 10);
//     engine.append(block_2);

//     let ret_1_at_block_2 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     let ret_2_at_block_2 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![None],
//             address: QueryAddress::KeyAddress(addr_2.to_string()),
//             block_number: None,
//         })
//         .unwrap();

//     assert_eq!(
//         ret_1_at_block_2.balances[0].unconstrained,
//         ((100_142 + 1000) * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_2_at_block_2.balances[0].unconstrained,
//         (100_000 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_1_at_block_2.balances[0].locked,
//         (142 * BYTE_SHANNONS).to_string()
//     );
//     assert_eq!(
//         ret_2_at_block_2.balances[0].locked,
//         (1000 * BYTE_SHANNONS).to_string()
//     );
// }

// #[test]
// fn test_get_udt_balance() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     // let addr_3 = "ckt1qyq98qe26z8eg8q0852h622m40s50swtqnrqndruht";

//     let engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 500_000, 300, 0, 0),
//         AddressData::new(addr_2, 0, 200, 100, 0),
//         // AddressData::new(addr_3, 600_000, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let ret_1 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![Some(SUDT_HASH.read().clone())],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     let ret_2 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![Some(SUDT_HASH.read().clone())],
//             address: QueryAddress::KeyAddress(addr_2.to_string()),
//             block_number: None,
//         })
//         .unwrap();

//     assert_eq!(ret_1.balances[0].unconstrained, 300.to_string());
//     assert_eq!(ret_2.balances[0].unconstrained, 300.to_string());
// }

// #[test]
// fn test_get_all_udt_balance() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     // let addr_3 = "ckt1qyq98qe26z8eg8q0852h622m40s50swtqnrqndruht";

//     let engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 500_000, 300, 0, 0),
//         AddressData::new(addr_2, 0, 200, 100, 0),
//         // AddressData::new(addr_3, 600_000, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let ret_1 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![],
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             block_number: None,
//         })
//         .unwrap();
//     let ret_2 = rpc
//         .get_balance(GetBalancePayload {
//             udt_hashes: hashset![],
//             address: QueryAddress::KeyAddress(addr_2.to_string()),
//             block_number: None,
//         })
//         .unwrap();

//     assert_eq!(ret_1.balances.len(), 2);
//     assert_eq!(ret_2.balances.len(), 2);
// }

// #[test]
// #[ignore]
// fn test_query_generic_transaction() {
//     let addr_1 = "ckt1qyqr79tnk3pp34xp92gerxjc4p3mus2690psf0dd70";
//     let addr_2 = "ckt1qyq2y6jdkynen2vx946tnsdw2dgucvv7ph0s8n4kfd";
//     // let addr_3 = "ckt1qyq98qe26z8eg8q0852h622m40s50swtqnrqndruht";

//     let engine = RpcTestEngine::init_data(vec![
//         AddressData::new(addr_1, 500_000, 300, 50, 0),
//         AddressData::new(addr_2, 50_000, 200, 0, 0),
//         // AddressData::new(addr_3, 600_000, 0, 0, 0, 0),
//     ]);

//     let rpc = engine.rpc();
//     let ret = rpc
//         .query_generic_transactions(QueryGenericTransactionsPayload {
//             address: QueryAddress::KeyAddress(addr_1.to_string()),
//             udt_hashes: hashset!(None),
//             from_block: Some(0),
//             to_block: Some(1),
//             limit: None,
//             offset: None,
//             order: None,
//         })
//         .unwrap();

//     println!("{:?}", ret);
// }
