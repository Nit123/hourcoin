use super::*;
use std::collections::HashSet;

pub struct Blockchain {
	pub blocks: Vec<Block>,
	unspent_outputs: HashSet<BlockHash>,
	difficulty: u128,
}

#[derive(Debug)]
pub enum BlockValidationErr {
	MismatchedIndex,
	InvalidHash,
	AchronologicalTimestamp,
	MismatchedPreviousHash,
	InvalidGenesisBlockFormat,
	InvalidInput,
	InsufficientInputValue,
	InvalidCoinbaseTransaction,
	InvalidDifficultyUpdate,
}

impl Blockchain {
	pub fn new () -> Self {
		Blockchain {
			blocks: vec![],
			unspent_outputs: HashSet::new(),
			difficulty: 23, // this value must be updated immediatelty after  
		}
	}

	pub fn new_with_diff (diff: u128) -> Self {
		Blockchain {
			blocks: vec![],
			unspent_outputs: HashSet::new(),
			difficulty: diff, // this value must be updated immediatelty after  
		}
	}

	pub fn update_difficulty (&mut self, diff:u128) -> Result<(), BlockValidationErr> {
		if self.difficulty < diff{
			return Err(BlockValidationErr::InvalidDifficultyUpdate);
		}
		else{
			self.difficulty = diff;
			Ok(())
		}

	}

	pub fn get_difficulty (&self) -> u128 {
		self.difficulty
	}

	pub fn update_with_block (&mut self, block:Block) -> Result<(), BlockValidationErr> {
		let i = self.blocks.len();
		// block index test
		if block.index != i as u32 {
			return Err(BlockValidationErr::MismatchedIndex);
		}
		// failed prescribed difficulty value...should make sure block is storing valid difficulty tho
		else if !block::check_blockhash(&block.hash(), self.difficulty) {
			return Err(BlockValidationErr::InvalidHash);
		}
		else if i != 0{
			// not genesis block
			let prev_block = &self.blocks[i-1];
			if block.timestamp <= prev_block.timestamp {
				return Err(BlockValidationErr::AchronologicalTimestamp);
			}
			else if block.prev_block_hash != prev_block.hash {
				return Err(BlockValidationErr::MismatchedPreviousHash);
			}
		}
		else{
			// genesis block
			if block.prev_block_hash != vec![0; 32] {
				return Err(BlockValidationErr::InvalidGenesisBlockFormat);
			}
		}

		if let Some((coinbase, transactions)) = block.transactions.split_first() {
			if !coinbase.is_coinbase() {
				return Err(BlockValidationErr::InvalidCoinbaseTransaction);
			}

			let mut block_spent:HashSet<BlockHash> = HashSet::new(); // input hashes that were spent in this block
			let mut block_created:HashSet<BlockHash> = HashSet::new(); // (unspent) output hashes generated by this block
			let mut total_fee = 0.0;

			for transaction in transactions {
				let input_hashes = transaction.input_hashes();

				// first condition is if there is a leftover input that didn't come from unspent output
				// second condition is that there is an input hash that has been used twice
				if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty(){
					return Err(BlockValidationErr::InvalidInput);
				}

				let input_sum = transaction.input_sum();
				let output_sum = transaction.output_sum();

				if output_sum > input_sum {
					return Err(BlockValidationErr::InsufficientInputValue);
				}

				let fee = input_sum - output_sum;
				total_fee += fee;

				block_spent.extend(input_hashes);
				block_created.extend(transaction.output_hashes())
			}

			if coinbase.output_sum() < total_fee {
				return Err(BlockValidationErr::InvalidCoinbaseTransaction);
			}
			else{
				block_created.extend(coinbase.output_hashes());
			}

			self.unspent_outputs.retain(|output| !block_spent.contains(output));
			self.unspent_outputs.extend(block_created);

		}

		self.blocks.push(block);
		
		Ok(())
	}
}