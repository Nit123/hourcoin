use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block {
	pub index: u32, // block index
	pub timestamp: u128, // timestamp of when block is created
	pub hash: Hash, // current block hash
	pub prev_block_hash: Hash, //prev block hash
	pub nonce: u64, // for mining
	pub transactions: Vec<Transaction>, // will change for transactions



}

impl Debug for Block {
	fn fmt (&self, f: &mut Formatter) -> fmt::Result {
		// write!(f, "Block [{}]: {} at: {} with: {} nonce: {}", 
		// 	&self.index, &hex::encode(&self.hash), &self.timestamp, &self.transactions.len(), &self.nonce
		// )
		write!(f, "[Block #{} - hash: {}, timestamp: {}, nonce: {}]: transactions: {}",
				&self.index, &hex::encode(&self.hash), &self.timestamp, &self.nonce, &self.transactions.len())
	}
}

impl Block { 
	pub fn new(index: u32, timestamp: u128,  prev_block_hash: Hash, transactions: Vec<Transaction>,) -> Self {
		Block {
			index, 
			timestamp, 
			hash: vec![0; 32], 
			prev_block_hash, 
			nonce: 0, 
			transactions,
		}
	}

	pub fn mine (&mut self, difficulty: u128){
		for nonce_attempt in 0..(u64::max_value()){
			self.nonce = nonce_attempt;
			let hash = self.hash();
			if check_blockhash(&hash, difficulty){
				self.hash = hash;
				return;
			}
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
		bytes.extend(self.transactions.iter()
									    .flat_map(|transaction| transaction.bytes())
									    .collect::<Vec<u8>>()
		);


		bytes
	}
}

pub fn check_blockhash (hash: &Hash, difficulty: u128) -> bool {
	difficulty > difficulty_bytes_as_u128(&hash)
}

