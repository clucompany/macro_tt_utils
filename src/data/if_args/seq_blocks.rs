
use alloc::vec::Vec;
use crate::data::if_args::ArgBlock;

pub trait SeqBlocks {
	type Block: ArgBlock;
	
	
	fn new() -> Self;
	fn with_capacity(u: usize) -> Self;
	
	fn push(&mut self, block: Self::Block);
	
	/*fn eq<F: FnMut() -> R, R>(&self, f: F);
	fn dont_eq<F: FnMut() -> R, R>(&self, f: F);*/
	fn is_equality(&self) -> bool;
}


impl<B> SeqBlocks for Vec<B> where B: ArgBlock {
	type Block = B;
	
	#[inline]
	fn new() -> Self {
		Vec::new()
	}
	
	#[inline]
	fn with_capacity(u: usize) -> Self {
		Vec::with_capacity(u)
	}
	/*
	fn eq<F: FnMut() -> R, R>(&self, mut f: F) {
		for a in self.iter() {
			if a.is_equality() == false {
				return;
			}
		}
		f();
	}
	fn dont_eq<F: FnMut() -> R, R>(&self, mut f: F) {
		for a in self.iter() {
			if a.is_equality() == false {
				f();
				return;
			}
		}
	}*/
	
	fn is_equality(&self) -> bool {
		for a in self.iter() {
			if a.is_equality() == false {
				return false;
			}
		}
		true
	}
	
	#[inline]
	fn push(&mut self, block: Self::Block) {
		Vec::push(self, block)
	}
}

