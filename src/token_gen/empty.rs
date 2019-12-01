
/*use proc_macro::Group;
use proc_macro::TokenTree;
use proc_macro::Delimiter;
use alloc::vec;*/
use proc_macro::TokenStream;

#[inline]
pub fn empty_token() -> TokenStream {
	let result = TokenStream::new();
	/*TokenStream::extend(
		&mut result, 
		
		vec![
			TokenTree::Group(
				Group::new(
					Delimiter::Brace,
				
					TokenStream::new(),
				)
				
			)
		]
	);*/
	result
}
