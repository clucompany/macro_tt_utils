
use alloc::string::String;
use crate::data::RecursionBuffer;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::Span;
use proc_macro::Spacing;
use alloc::string::ToString;

macro_rules! throw_error {
	( $sp: expr, $s: expr ) => {
		return crate::diagnostics::throw_error($sp, $s).into();
	};
}

macro_rules! throw_warning {
	( $sp: expr, $s: expr ) => {
		return crate::diagnostics::throw_warning($sp, $s).into();
	};
}

macro_rules! throw_note {
	( $sp: expr, $s: expr ) => {
		return crate::diagnostics::throw_note($sp, $s).into();
	};
}

macro_rules! throw_help {
	( $sp: expr, $s: expr ) => {
		return crate::diagnostics::throw_help($sp, $s).into();
	};
}

#[derive(Debug)]
pub enum MacrosType {
	RootMacros,
	Default,
}


#[derive(Debug)]
pub struct DecodeDiagToken<I> where I: Iterator<Item = TokenTree> {
	buffer: RecursionBuffer<I, TokenTree>,
}


impl<I> DecodeDiagToken<I> where I: Iterator<Item = TokenTree> {
	#[inline]
	pub (crate) const fn __new(buffer: RecursionBuffer<I, TokenTree>) -> Self {
		Self {
			buffer: buffer,
		}
	}
	
	#[inline]
	pub (crate) fn expect_string<N: FnMut(Self, String, MacrosType, Span) -> TokenStream>(mut self, mut n: N, mut span: Span) -> TokenStream {
		self.buffer.next();
		
		let type_macros: MacrosType;
		let string = match *self.buffer {
			Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
				('@', Spacing::Alone) => {
					span = punct.span();
					self.buffer.next();
										
					match *self.buffer {
						Some(TokenTree::Ident(ref i)) => match i.to_string().as_str() {
							"root" => { //next string
								span = i.span();
								self.buffer.next();
								
								match *self.buffer {
									Some(TokenTree::Literal(ref lit)) => {
										let mut string = lit.to_string();
										span = lit.span();
										
										//del end symbol
										match string.pop() {
											Some('"') => {},
											Some(_) => throw_error!(span, "this was expected to be a string"),
											None => throw_error!(span, "this was expected to be a string"),
										}
										
										match string.chars().next() {
											Some('"') => {},
											Some(_) => throw_error!(span, "this was expected to be a string"),
											None => throw_error!(span, "this was expected to be a string"),
										}
										
										type_macros = MacrosType::RootMacros;
										unsafe {
											string.get_unchecked(1..).to_string()
										}
									},
									Some(ref a) => throw_error!(a.span(), "unsupported input argument type, expected '@root', 'string'"),
									_ => throw_error!(span, "an unexpected break in the description of the input arguments, expected '@root', 'string'"),
								}
								
							}, //OK
							_ => throw_error!(i.span(), "undefined macro type, expected 'root'"),
						},
						Some(ref a) => throw_error!(a.span(), "undefined macro type, expected '@root'"),
						_ => throw_error!(span, "undefined macro type, expected '@root'"),
					}
				},
				(_r, _s) => throw_error!(punct.span(),  "unsupported input argument type, expected '@root'")
			},
			/*Some(TokenTree::Group(ref group)) => {
				let mut iter = group.stream().into_iter();
				span = group.span();
				
				*self.buffer = iter.next();
				let result = match *self.buffer {
					Some(TokenTree::Ident(ref i)) => i.to_string(),
					Some(ref a) => throw_error!(a.span(), "this was expected to be one of the input elements of the macro"),
					_ => throw_error!(span, "an unexpected break in the description of the input arguments, expected '$a'"),
				};
				
				*self.buffer = iter.next();
				if let Some(ref a) = *self.buffer {
					throw_error!(a.span(), "no additional arguments were expected");
				}
				
				result
			},*/
			Some(TokenTree::Literal(ref lit)) => {
				let mut string = lit.to_string();
				span = lit.span();
				
				//del end symbol
				match string.pop() {
					Some('"') => {},
					Some(_) => throw_error!(span, "this was expected to be a string"),
					None => throw_error!(span, "this was expected to be a string"),
				}
				
				match string.chars().next() {
					Some('"') => {},
					Some(_) => throw_error!(span, "this was expected to be a string"),
					None => throw_error!(span, "this was expected to be a string"),
				}
				
				type_macros = MacrosType::Default;
				unsafe {
					string.get_unchecked(1..).to_string()
				}
			},
			Some(ref a) => throw_error!(a.span(), "unsupported input argument type, expected '@root', 'string'"),
			_ =>  throw_error!(span, "an unexpected break in the description of the input arguments, expected '@root', 'string'"),
		};
		
		
		
		n(self, string, type_macros, span)
	}
}

#[inline]
pub fn new_decode_diag_token(token: TokenStream) -> DecodeDiagToken<impl Iterator<Item = TokenTree>> {
	let iter = token.into_iter();
	
	DecodeDiagToken::__new(RecursionBuffer::new(iter))
}




pub fn throw_compile_error(input: TokenStream) -> TokenStream {
	let decoder = new_decode_diag_token(input);
	
	decoder.expect_string(|mut decoder, data, ttype, _span| {

		decoder.buffer.next();
		match *decoder.buffer {
			Some(ref a) => throw_error!(a.span(), "no additional arguments were expected"),
			_ => {
				let span = match ttype {
					MacrosType::Default => Span::mixed_site(),
					MacrosType::RootMacros => Span::mixed_site().source(),
				};
				throw_error!(span, data)
			},
		}
		
	}, Span::call_site())
}


pub fn throw_compile_warning(input: TokenStream) -> TokenStream {
	let decoder = new_decode_diag_token(input);
	
	decoder.expect_string(|mut decoder, data, ttype, _span| {

		decoder.buffer.next();
		match *decoder.buffer {
			Some(ref a) => throw_error!(a.span(), "no additional arguments were expected"),
			_ => {
				let span = match ttype {
					MacrosType::Default => Span::mixed_site(),
					MacrosType::RootMacros => Span::mixed_site().source(),
				};
				throw_warning!(span, data)
			},
		}
		
	}, Span::call_site())
}


pub fn throw_compile_note(input: TokenStream) -> TokenStream {
	let decoder = new_decode_diag_token(input);
	
	decoder.expect_string(|mut decoder, data, ttype, _span| {

		decoder.buffer.next();
		match *decoder.buffer {
			Some(ref a) => throw_error!(a.span(), "no additional arguments were expected"),
			_ => {
				let span = match ttype {
					MacrosType::Default => Span::mixed_site(),
					MacrosType::RootMacros => Span::mixed_site().source(),
				};
				throw_note!(span, data)
			},
		}
		
	}, Span::call_site())
}

pub fn throw_compile_help(input: TokenStream) -> TokenStream {
	let decoder = new_decode_diag_token(input);
	
	decoder.expect_string(|mut decoder, data, ttype, _span| {

		decoder.buffer.next();
		match *decoder.buffer {
			Some(ref a) => throw_error!(a.span(), "no additional arguments were expected"),
			_ => {
				let span = match ttype {
					MacrosType::Default => Span::mixed_site(),
					MacrosType::RootMacros => Span::mixed_site().source(),
				};
				throw_help!(span, data)
			},
		}
		
	}, Span::call_site())
}


