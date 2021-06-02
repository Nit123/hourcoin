use blockchainlib::*;

fn main() {
	let difficulty = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

	// example of genesis block with two coinbase transactions and example of adding to blockchain/mining
	let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![Transaction {
																		inputs: vec![],
																		outputs: vec![
																			transaction::Output{
																				value: 50,
																				to_addr: "Alice".to_owned(),
																			},
																			transaction::Output{
																				value: 7,
																				to_addr: "Bob".to_owned(),
																			}]}],);

	genesis_block.mine(difficulty);
	println!("Mined genesis block: \n {:?}", &genesis_block);

	let last_hash = genesis_block.hash.clone();

	let mut blockchain = Blockchain::new_with_diff(difficulty);

	blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

	 let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 36,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ],);

	block.mine(blockchain.get_difficulty());

    println!("Mined block\n {:?}", &block);

    blockchain.update_with_block(block).expect("Failed to add block");

}