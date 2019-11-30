#![feature(proc_macro_diagnostic)]
#![feature(const_fn)]
#![feature(proc_macro_span)]
#![feature(proc_macro_mixed_site)]

#![no_std]

extern crate alloc;
extern crate proc_macro;

use proc_macro::TokenStream;

mod proc_macro_fn {
	pub mod diag;
	pub mod span;
	pub mod tt_ident;
}

pub (crate) mod diagnostics {
	mod throw_error;
	pub use self::throw_error::*;
	
	mod throw_warning;
	pub use self::throw_warning::*;
	
	mod throw_note;
	pub use self::throw_note::*;
	
	mod throw_help;
	pub use self::throw_help::*;
}

pub (crate) mod data {
	pub mod if_args {
		mod block;
		pub use self::block::*;
		
		mod eq_seq_blocks;
		pub use self::eq_seq_blocks::*;
		
		mod seq_blocks;
		pub use self::seq_blocks::*;
		
		mod cond_seq_blocks;
		pub use self::cond_seq_blocks::*;
		
		mod reverse_eq_block;
		pub use self::reverse_eq_block::*;
	}
	
	pub mod span {
		mod element;
		pub use self::element::*;
		
		mod into;
		pub use self::into::*;
	}
	
	mod macro_string;
	pub use self::macro_string::*;
	
	mod recursion_buffer;
	pub use self::recursion_buffer::*;
}

pub (crate) mod token_gen {
	mod empty;
	pub use self::empty::*;
	
	mod deref_span;
	pub use self::deref_span::*;
}



#[proc_macro]
#[inline(always)]
pub fn tt_ident(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::tt_ident::tt_ident(input)
}


#[proc_macro]
#[inline(always)]
pub fn deref_span(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::span::deref_span(input)
}

#[proc_macro]
#[inline(always)]
pub fn throw_compile_error(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::diag::throw_compile_error(input)
}

#[proc_macro]
#[inline(always)]
pub fn throw_compile_warning(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::diag::throw_compile_warning(input)
}

#[proc_macro]
#[inline(always)]
pub fn throw_compile_note(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::diag::throw_compile_note(input)
}

#[proc_macro]
#[inline(always)]
pub fn throw_compile_help(input: TokenStream) -> TokenStream {
	crate::proc_macro_fn::diag::throw_compile_help(input)
}


