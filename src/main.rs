use blockchainlib::*;

fn main() {
	let mut block = Block::new(0, 0, vec![0; 32], 0, "Gensis block created".to_owned());
	let h = block.hash();
	block.hash = h;

	println!("{:?}", &block);

}