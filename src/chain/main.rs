use blockchainlib::*;
use rand::Rng; // used to generate random u128 numbers for timestamp examples

fn main() {
	let difficulty = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let mut rng = rand::thread_rng();

	// example of genesis block with two coinbase transactions and example of adding to blockchain/mining
	let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![Transaction {
																		inputs: vec![],
																		outputs: vec![
																			transaction::Output{
																				value: 1.5,
																				to_addr: "Alice".to_owned(),
                                                                                timestamp: now()
																			},
																			transaction::Output{
																				value: 0.5,
																				to_addr: "Bob".to_owned(),
                                                                                timestamp: now()
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
                    value: 2.0,
                    timestamp: rng.gen(),
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
                    value: 0.25,
                    timestamp: rng.gen(),
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 0.5,
                    timestamp: rng.gen(),
                },
            ],
        },
    ],);

	block.mine(blockchain.get_difficulty());

    println!("Mined block\n {:?}", &block);

    blockchain.update_with_block(block).expect("Failed to add block");
    

}