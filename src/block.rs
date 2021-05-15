use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block {
	pub index: u32, // block index
	pub timestamp: u128, // timestamp of when block is created
	pub hash: BlockHash, // current block hash
	pub prev_block_hash: BlockHash, //prev block hash
	pub nonce: u64, // for mining
	pub payload: String, // will change for transactions


}

impl Debug for Block {
	fn fmt (&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Block [{}]: {} at: {} with: {}", 
			&self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload
		)
	}
}

impl Block { 
	pub fn new(index: u32, timestamp: u128,  prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
		Block {
			index, 
			timestamp, 
			hash: vec![0; 32], 
			prev_block_hash, 
			nonce, 
			payload,
		}
	}
}

impl Hashable for Block {
	fn bytes (&self) -> Vec<u8> {
		let mut bytes = vec![];

		bytes.extend(&u32_bytes(&self.index));
		bytes.extend(&u128_bytes(&self.timestamp));
		bytes.extend(&self.prev_block_hash);
		bytes.extend(&u64_bytes(&self.nonce));
		bytes.extend(self.payload.as_bytes());

		bytes
	}
}