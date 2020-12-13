use sha2::{Digest, Sha256};
use hex;
use chrono::prelude::*;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    time_stamp: String,
    data: i32,
    hash: String,
    prev_hash: String,
    nonce: String,
    difficulty: u32,
}

impl Block {
    fn new(old_block: &Block, data: i32, dif: u32) -> Self {
        let index = old_block.index + 1;
        let t = Utc::now();
        let time_stamp = format!("{}", t);
        let prev_hash = &*old_block.hash.clone();
        let mut block = Block { index, time_stamp, data, hash: "".to_string(), prev_hash: prev_hash.to_string(), nonce: "".to_string(), difficulty: dif};
        let mut i = 0u32;
        loop {
            i+=1;
            block.nonce = format!("{}", i);
            if !is_hash_valid(block.calculate_hash(), block.difficulty) {
                println!("{} do more work !", block.calculate_hash());
                
                continue;
            } else {
                println!("{} work done !", block.calculate_hash());
                block.hash = block.calculate_hash();
                break;
            }
        }
        block

    }
    fn is_block_valid(&self, old_block: &Block) -> bool {
        if old_block.index + 1 != self.index { return false; }
        if old_block.hash != self.prev_hash { return false; }
        if self.calculate_hash() != self.hash { return false; }
        true
    }
    fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}{}{}", self.index, self.time_stamp, self.data, self.prev_hash, self.nonce);
        let mut h = Sha256::new();
        h.update(record.as_bytes());
        let hashed = h.finalize();

        hex::encode(hashed)
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result{
        println!("[");
        for block in &self.blocks {
            println!("  {{");
            println!("    \"Index\": {},", block.index);
            println!("    \"Timestamp\": \"{}\",", block.time_stamp);
            println!("    \"Data\": {},", block.data);
            println!("    \"Hash\": \"{}\",", block.hash);
            println!("    \"PrevHash\": \"{}\"", block.prev_hash);
            if block.index as usize == self.blocks.len() - 1 {
            println!("  }}");
            } else {
                println!("  }},");
            }
        }
        println!("]");

        Ok(())
    }
}
#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
    difficulty: u32,
}
impl Blockchain {
    fn init(difficulty: u32) -> Self {
        let t = Utc::now();
        let block = Block {index: 0, time_stamp: format!("{}", t), data: 0, hash: "".to_string(), prev_hash: "".to_string(), difficulty, nonce: "".to_string()};
        Self {blocks: vec![block], difficulty}
    }
    fn replace_chain(&mut self, new_blocks: Vec<Block>) {
        if new_blocks.len() > self.blocks.len() {
            self.blocks = new_blocks;
        }
    }
    fn add_block(&mut self, data: i32) {
        if self.blocks.len() < 1 {
            return;
        }
        let block = Block::new(&self.blocks[self.blocks.len() - 1], data, self.difficulty);

        if block.is_block_valid(&self.blocks[self.blocks.len() - 1]) {
            let mut new_blockchain = self.clone();
            new_blockchain.blocks.push(block);
            self.replace_chain(new_blockchain.blocks);
        }
    }
}

fn is_hash_valid(hash: String, difficulty: u32) -> bool {
    let mut prefix = String::new();
    for _ in 0..difficulty {
        prefix.push('0')
    }
    hash.starts_with(&prefix)
}

fn main() {
    let mut block_chain = Blockchain::init(1);
    block_chain.add_block(5);
    block_chain.add_block(6);
    block_chain.add_block(7);
    block_chain.add_block(8);
    println!("{}", block_chain);
}
