

use alloc::vec::Vec;
use crate::data::if_args::EqSeqBlocks;
use crate::data::if_args::SeqBlocks;

pub trait ConditionSeqBlocks {
	type SeqB: SeqBlocks;
	
	fn new() -> Self;
	fn with_capacity(u: usize) -> Self;
	
	fn new_seq_blocs(&mut self) -> &mut Self::SeqB;
	
	fn is_equality(&self) -> EqSeqBlocks;
}

impl<SeqB> ConditionSeqBlocks for Vec<SeqB> where SeqB: SeqBlocks {
	type SeqB = SeqB;
	
	#[inline]
	fn new() -> Self {
		Vec::new()
	}
	
	#[inline]
	fn with_capacity(u: usize) -> Self {
		Vec::with_capacity(u)
	}
	#[inline]
	fn new_seq_blocs(&mut self) -> &mut SeqB {
		self.push(SeqB::with_capacity(3));
		let len = self.len();
		unsafe {
			self.get_unchecked_mut(len-1) as _
		}
	}
	
	#[inline]
	fn is_equality(&self) -> EqSeqBlocks {
		let mut iter = self.iter();
		let mut _a = match iter.next() {
			None => return EqSeqBlocks::empty_array(), //массив пуст
			Some(a) => match a.is_equality() {
				true => return EqSeqBlocks::true_eq(), //успех хотябы один
				false => {}, //continue
			}
		};
		
		loop {
			_a = match iter.next() {
				None => return EqSeqBlocks::false_eq(),
				Some(a) => match a.is_equality() {
					true => return EqSeqBlocks::true_eq(), //успех хотябы один
					false => {}, //continue
				}
			};
		}
	}
}
