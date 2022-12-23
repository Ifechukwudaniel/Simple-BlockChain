extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receive: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        return chain;
    }

    pub fn generate_new_block() {}
}
