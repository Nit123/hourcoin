use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output {
	pub to_addr: Address,
	pub value: f64,
	pub timestamp: u128,
}

impl Hashable for Output {
	fn bytes (&self) -> Vec<u8> {
		let mut bytes = vec![];
		bytes.extend(self.to_addr.as_bytes());
		bytes.extend(&self.value.to_be_bytes());
		bytes.extend(&self.timestamp.to_be_bytes());

		bytes
	}
}

pub struct Transaction {
	pub inputs: Vec<Output>,
	pub outputs: Vec<Output>,
}

impl Transaction {
	pub fn input_sum (&self) -> f64 {
		self.inputs.iter()
			.map(|input| input.value)
			.sum()
	}

	pub fn output_sum (&self) -> f64 {
		self.outputs.iter()
			.map(|output| output.value)
			.sum()
	}

	pub fn input_hashes (&self) -> HashSet<BlockHash> {
		self.inputs.iter()
			.map(|input| input.hash())
			.collect::<HashSet<BlockHash>>()
	}

	pub fn output_hashes (&self) -> HashSet<BlockHash> {
		self.outputs.iter()
			.map(|output| output.hash())
			.collect::<HashSet<BlockHash>>()
	}

	pub fn is_coinbase (&self) -> bool {
		(self.inputs.len() == 0) && (self.output_sum() == 2.0)
	}
}

impl Hashable for Transaction {
	fn bytes (&self) -> Vec<u8> {
		let mut bytes = vec![];

		bytes.extend(self.inputs.iter()
								.flat_map(|input| input.bytes())
								.collect::<Vec<u8>>());
		bytes.extend(self.outputs.iter()
								.flat_map(|output| output.bytes())
								.collect::<Vec<u8>>());

		bytes
	}
}