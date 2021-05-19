use super::*;

pub struct Blockchain {
	pub blocks: Vec<Block>
}

impl Blockchain {
	pub fn verify (&self) -> bool {
		for (i, block) in self.blocks.iter().enumerate() {
			// block index test
			if block.index != i as u32 {
				println!("Index mismatch, block rejected: {} != {}", &block.index, &i,);
				return false;
			}
			// failed prescribed difficulty value...should make sure block is storing valid difficulty tho
			else if !block::check_difficulty(&block.hash(), block.difficulty) {
				println!("Difficulty failed");
				return false;
			}
			else if i != 0{
				// not genesis block
				let prev_block = &self.blocks[i-1];
				if block.timestamp <= prev_block.timestamp {
					println!("Time value incorrect");
					return false;
				}
				else if block.prev_block_hash != prev_block.hash {
					println!("Previous hash mismatch");
					return false;
				}
			}
			else{
				// genesis block
				if block.prev_block_hash != vec![0; 32] {
					println!("Genesis block invalid by prev hash");
					return false;
				}
			}
		}

		true
	}
}