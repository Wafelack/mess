use chrono::prelude::*;
use hex;
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct Block<T>
where
    T: std::fmt::Debug,
    T: std::default::Default,
    T: std::clone::Clone,
{
    index: u32,
    time_stamp: String,
    data: T,
    hash: String,
    prev_hash: String,
    nonce: String,
    difficulty: u32,
}

impl<T> Block<T>
where
    T: std::fmt::Debug,
    T: std::default::Default,
    T: std::clone::Clone,
{
    pub fn new(old_block: &Block<T>, data: T, dif: u32) -> Self {
        let index = old_block.index + 1;
        let t = Utc::now();
        let time_stamp = format!("{}", t);
        let prev_hash = &*old_block.hash.clone();
        let mut block = Block {
            index,
            time_stamp,
            data,
            hash: "".to_string(),
            prev_hash: prev_hash.to_string(),
            nonce: "".to_string(),
            difficulty: dif,
        };
        let mut i = 0u32;
        loop {
            i += 1;
            block.nonce = format!("{}", i);
            if !is_hash_valid(block.calculate_hash(), block.difficulty) {
                continue;
            } else {
                block.hash = block.calculate_hash();
                break;
            }
        }
        block
    }
    pub fn is_block_valid(&self, old_block: &Block<T>) -> bool {
        if old_block.index + 1 != self.index {
            return false;
        }
        if old_block.hash != self.prev_hash {
            return false;
        }
        if self.calculate_hash() != self.hash {
            return false;
        }
        true
    }
    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{:?}{}{}",
            self.index, self.time_stamp, self.data, self.prev_hash, self.nonce
        );
        let mut h = Sha256::new();
        h.update(record.as_bytes());
        let hashed = h.finalize();

        hex::encode(hashed)
    }
}

impl<T> std::fmt::Display for Blockchain<T>
where
    T: std::fmt::Debug,
    T: std::default::Default,
    T: std::clone::Clone,
{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("[");
        for block in &self.blocks {
            println!("  {{");
            println!("    \"Index\": {},", block.index);
            println!("    \"Timestamp\": \"{}\",", block.time_stamp);
            println!("    \"Data\": {:?},", block.data);
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
pub struct Blockchain<T>
where
    T: std::fmt::Debug,
    T: std::default::Default,
    T: std::clone::Clone,
{
    blocks: Vec<Block<T>>,
    difficulty: u32,
}
impl<T> Blockchain<T>
where
    T: std::fmt::Debug,
    T: std::default::Default,
    T: std::clone::Clone,
{
    pub fn init(difficulty: u32) -> Self {
        let t = Utc::now();
        let default: T = Default::default();
        let block = Block {
            index: 0,
            time_stamp: format!("{}", t),
            data: default,
            hash: "".to_string(),
            prev_hash: "".to_string(),
            difficulty,
            nonce: "".to_string(),
        };
        Self {
            blocks: vec![block],
            difficulty,
        }
    }
    pub fn replace_chain(&mut self, new_blocks: Vec<Block<T>>) {
        if new_blocks.len() > self.blocks.len() {
            self.blocks = new_blocks;
        }
    }
    /// Adds a blocks to the block chain by calling block::new and checks if the produced block is valid
    pub fn add_block(&mut self, data: T) {
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
/// Tests if hash is valid with difficulty
pub fn is_hash_valid(hash: String, difficulty: u32) -> bool {
    let mut prefix = String::new();
    for _ in 0..difficulty {
        prefix.push('0')
    }
    hash.starts_with(&prefix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hashes() {
        let mut block_chain = Blockchain::<bool>::init(1);
        block_chain.add_block(true);
        block_chain.add_block(false);
        assert_eq!(
            block_chain.blocks[block_chain.blocks.len() - 1].prev_hash,
            block_chain.blocks[block_chain.blocks.len() - 2].hash
        );
    }

    #[test]
    fn validation() {
        let mut block_chain = Blockchain::init(1);
        block_chain.add_block(5);
        block_chain.add_block(6);
        assert!(block_chain.blocks[block_chain.blocks.len() - 1]
            .is_block_valid(&block_chain.blocks[block_chain.blocks.len() - 2]));
    }
    #[test]
    fn display() {
        let mut blockchain = Blockchain::<Vec<u8>>::init(1);
        blockchain.add_block("Hello, World !".to_string().into_bytes());
        blockchain.add_block("Goodbye !".to_string().into_bytes());
        println!("{}", blockchain);
    }
}
