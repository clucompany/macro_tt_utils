
use proc_macro::Span;
use proc_macro::TokenStream;
use core::iter::FromIterator;
use alloc::vec::Vec;

pub fn deref_span(input: TokenStream) -> TokenStream {
	let mut vec = Vec::with_capacity(7);
	for mut a in input {
		a.set_span(Span::mixed_site().source());
		vec.push(a);
	}
	
	TokenStream::from_iter(vec)
}

