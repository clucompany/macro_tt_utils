
use alloc::string::String;
use crate::data::span::SpanElement;
use proc_macro::TokenStream;
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ThrowWarning<S, I> where S: SpanElement, I: Into<String> + Debug {
	sp: S,
	data: I
}

impl<S, I> ThrowWarning<S, I> where S: SpanElement, I: Into<String> + Debug {
	#[inline]
	pub const fn new(sp: S, t: I) -> Self {
		Self {
			sp: sp,
			data: t
		}
	}
	
	pub fn raw_throw(self) {
		self.sp.warning(self.data).emit();
	}
	
	#[inline]
	pub fn throw(self) -> TokenStream {
		self.raw_throw();
		crate::token_gen::empty_token()
	}
}

#[inline]
pub fn throw_warning<S: SpanElement, I: Into<String> + Debug>(sp: S, str: I) -> TokenStream {
	ThrowWarning::new(sp, str).throw()
}

