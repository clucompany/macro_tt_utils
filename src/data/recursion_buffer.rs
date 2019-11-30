
use core::ops::DerefMut;
use core::ops::Deref;

#[derive(Debug)]
pub struct RecursionBuffer<I, T> where I: Iterator<Item = T> {
	iter: I,
	a: Option<T>,
}

impl<I, T> Deref for RecursionBuffer<I, T> where I: Iterator<Item = T> {
	type Target = Option<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.a
	}
}

impl<I, T> DerefMut for RecursionBuffer<I, T> where I: Iterator<Item = T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.a
	}
}

impl<I, T> RecursionBuffer<I, T> where I: Iterator<Item = T> {
	#[inline]
	pub const fn new(iter: I) -> Self {
		Self {
			iter: iter,
			a: None,
		}
	}
	
	#[inline(always)]
	pub fn next(&mut self) {
		self.a = self.iter.next();
	}
}
