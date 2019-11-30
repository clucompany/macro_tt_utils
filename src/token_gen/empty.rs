
use proc_macro::Group;
use proc_macro::TokenTree;
use proc_macro::TokenStream;
use proc_macro::Delimiter;
use alloc::vec;

pub fn empty_token() -> TokenStream {
	let mut result = TokenStream::new();
	TokenStream::extend(
		&mut result, 
		
		vec![
			TokenTree::Group(
				Group::new(
					Delimiter::Brace,
				
					TokenStream::new(),
				)
				
			)
		]
	);
	result
}
