use blockchainlib::*;

fn main() {
	let difficulty = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
	let mut block = Block::new(0, now(), vec![0; 32], 0, "Gensis block created".to_owned(), difficulty);

	block.mine();
	println!("Mined genesis block {:?}", &block);

	let mut last_hash = block.hash.clone();

	let mut blockchain = Blockchain {
		blocks: vec![block],
	};

	for i in 1..=10 {
		let mut block = Block::new(i, now(), last_hash, 0, "block created".to_owned(), difficulty);

		block.mine();
		last_hash = block.hash.clone();
		println!("Mined block {:?}", &block);
		if blockchain.verify() {
			blockchain.blocks.push(block);
		}
		else{
			return;
		}
	}

	// test to show verify can check errors
	blockchain.blocks[3].index = 4;
	println!("Verify: {}", &blockchain.verify());
}