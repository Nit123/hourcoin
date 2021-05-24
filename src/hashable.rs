use super::*;
pub trait Hashable {
	fn bytes (&self) -> Vec<u8>;

	fn hash (&self) -> BlockHash {
		crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
	}
}