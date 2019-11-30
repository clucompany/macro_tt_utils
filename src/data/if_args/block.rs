
use core::fmt::Debug;
use crate::data::if_args::ReverseEqBlock;

pub trait ArgBlock where Self: From<(ReverseEqBlock, <Self as ArgBlock>::A0, <Self as ArgBlock>::A1)> {
	type A0: PartialEq<<Self as ArgBlock>::A1>;
	type A1;
	
	fn as_arg0(&self) -> &Self::A0;
	fn as_arg1(&self) -> &Self::A1;
	
	fn is_equality(&self) -> bool;
}


#[derive(Debug)]
pub struct UniversalArgBlock<A, T> where A: PartialEq<T> {
	arg0: A,
	arg1: T,
	
	is_reverse: ReverseEqBlock,
}

impl<A, T> From<(ReverseEqBlock, A, T)> for UniversalArgBlock<A, T> where A: PartialEq<T> {
	#[inline(always)]
	fn from((r, arg0, arg1):(ReverseEqBlock, A, T)) -> Self {
		Self::new(r, arg0, arg1)
	}
}

impl<A0, A1> ArgBlock for UniversalArgBlock<A0, A1> where A0: PartialEq<A1> {
	type A0 = A0;
	type A1 = A1;
	
	#[inline(always)]
	fn as_arg0(&self) -> &Self::A0 {
		&self.arg0
	}
	
	#[inline(always)]
	fn as_arg1(&self) -> &Self::A1 {
		&self.arg1
	}
	
	#[inline]
	fn is_equality(&self) -> bool {
		if self.is_reverse == true {
			self.arg0 != self.arg1
		}else {
			self.arg0 == self.arg1
		}
	}
}

impl<A, T> UniversalArgBlock<A, T> where A: PartialEq<T> {
	#[inline]
	pub const fn new(is_reverse: ReverseEqBlock, arg0: A, arg1: T) -> Self {
		Self {
			arg0: arg0,
			arg1: arg1,
			
			is_reverse: is_reverse,
		}
	}
}