
use proc_macro::TokenStream;

#[inline]
pub fn deref_span(input: TokenStream) -> TokenStream {
	crate::token_gen::deref_span(input)
}

