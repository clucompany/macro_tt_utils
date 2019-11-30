
use alloc::vec::Vec;
use alloc::string::String;
use crate::data::RecursionBuffer;
use crate::data::if_args::UniversalArgBlock;
use proc_macro::Span;
use proc_macro::TokenStream;
use crate::data::if_args::ConditionSeqBlocks;
use crate::data::if_args::EqSeqBlocks;
use proc_macro::TokenTree;
use crate::data::if_args::SeqBlocks;
use crate::data::MacroString;
use crate::data::if_args::ReverseEqBlock;
use proc_macro::Spacing;
use goto::gpoint;
use alloc::format;
use alloc::string::ToString;

macro_rules! throw_error {
	( $sp: expr, $s: expr ) => {
		return crate::diagnostics::throw_error($sp, $s).into();
	};
}



#[derive(Debug)]
pub (crate) struct DecodeTTIFIdentToken<I> where I: Iterator<Item = TokenTree> {
	buffer: RecursionBuffer<I, TokenTree>,
}



impl<Itt> DecodeTTIFIdentToken<Itt> where Itt: Iterator<Item = TokenTree> {
	#[inline]
	const fn __new(buffer: RecursionBuffer<Itt, TokenTree>) -> Self {
		Self {
			buffer: buffer,
		}
	}
	
	#[inline]
	pub fn expect_ident<T: FnMut(Self, String, Span) -> TokenStream>(mut self, expected: &'static str, mut t: T, mut err: Span) -> TokenStream {
		self.buffer.next();

		match *self.buffer {
			Some(TokenTree::Ident(ref ident)) => {
				err = ident.span();
				let str = ident.to_string();
				
				t(self, str, err)
			},
			Some(ref a) => throw_error!(a.span(), format!("unknown input `{}`, expected '{}'", a, expected)),
			_ => throw_error!(err, format!("empty input, expected '{}'", expected)),
		}
	}
	
	//ConditionSeqBlocks<SeqB = SeqB>, SeqB: SeqBlocks<Block = A>, A: ArgBlock
	#[inline]
	pub fn decode_arguments<T: FnMut(Self, &mut D, Span) -> TokenStream, D: ConditionSeqBlocks>(mut self, condition_seq_blocks: &mut D, mut t: T, mut span: Span) -> TokenStream where <<D as ConditionSeqBlocks>::SeqB as SeqBlocks>::Block: From<(ReverseEqBlock, MacroString, MacroString)>  {
		let mut reverse_data_default: ReverseEqBlock = Default::default();
		
		gpoint!['decode_start_args:
		
			self.buffer.next();
			match *self.buffer {
				Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
					('!', Spacing::Alone) => {
						span = punct.span();
						reverse_data_default.to_one();
						
						continue 'decode_start_args;
					},
					(ch, _a) => throw_error!(punct.span(), format!("undefined input `{}`, expected void or '!'", ch)),
				},
				Some(TokenTree::Group(ref group)) => {
					let mut args_section = condition_seq_blocks.new_seq_blocs();
					span = group.span();
					
					// ($a == $t)
					// decode $a
					let mut iter = group.stream().into_iter();
					
					gpoint!['decode_conditions: 
						let mut reverse_data = reverse_data_default.clone();
						let mut is_ease_comparison = false;
						
						*self.buffer = iter.next();
						let arg0 = match *self.buffer {
							Some(TokenTree::Group(ref group)) => {
								span = group.span();
								let mut iter = group.stream().into_iter();
								
								*self.buffer = iter.next();
								let result = match *self.buffer {
									Some(TokenTree::Ident(ref i)) => {
										span = i.span();
										i.into()
									},
									Some(ref a) => throw_error!(a.span(), "this was expected to be one of the input elements of the macro"),
									_ => throw_error!(span, "an unexpected break in the description of the input arguments, expected '$a'"),
								};
								
								*self.buffer = iter.next();
								if let Some(ref a) = *self.buffer {
									throw_error!(a.span(), "no additional arguments were expected");
								}
								
								result
							},
							Some(TokenTree::Literal(ref lit)) => {
								let mut string = lit.to_string();
								span = lit.span();					
								is_ease_comparison = true;
								
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
								
								
								unsafe { MacroString::literal(string) }
							},
							Some(ref a) => throw_error!(a.span(), "unsupported input argument type for condition, expected '$a' or 'String'"),
							_ =>  throw_error!(span, "empty condition record, expected '$a == $b'"),
						};
						
						//== !=
						*self.buffer = iter.next();
						match *self.buffer {
							Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
								('=', Spacing::Joint) => {
									span = punct.span();
									
									*self.buffer = iter.next();
									match *self.buffer {
										Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
											('=', Spacing::Alone) => {
												span = punct.span();
											}, // ==
											
											ref _a => throw_error!(punct.span(), "undefined condition, expected '==', '!='"),
										},
										Some(ref a) => throw_error!(a.span(), "undefined condition, expected '==', '!='"),
										_ => throw_error!(span, "empty condition record, expected '==', '!='"),
									}
								},
								('!', Spacing::Joint) => {
									span = punct.span();
									
									*self.buffer = iter.next();
									match *self.buffer {
										Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
											('=', Spacing::Alone) => {
												span = punct.span();
												reverse_data.next();
											}, // !=
											
											ref _a => throw_error!(punct.span(), "undefined condition, expected '==', '!='"),
										},
										Some(ref a) => throw_error!(a.span(), "undefined condition, expected '==', '!='"),
										_ => throw_error!(span, "undefined condition, expected '==', '!='"),
									}
								},
								ref _a => throw_error!(punct.span(), "undefined condition, expected '==', '!='"),
							},
							
							Some(ref tree) => throw_error!(tree.span(), "undefined condition, expected '==', '!='"),
							_ => throw_error!(span, "empty condition record, expected '==', '!='"),
						}
						
						//arg1
						*self.buffer = iter.next();
						let arg1 = match *self.buffer {
							Some(TokenTree::Group(ref group)) => {
								span = group.span();
								let mut iter = group.stream().into_iter();
								
								*self.buffer = iter.next();
								let result = match *self.buffer {
									Some(TokenTree::Ident(ref i)) => {
										span = i.span();
										i.into()
									},
									Some(ref a) => throw_error!(a.span(), "this was expected to be one of the input elements of the macro"),
									_ => throw_error!(span, "an unexpected break in the description of the input arguments, expected '$a'"),
								};
								
								*self.buffer = iter.next();
								if let Some(ref a) = *self.buffer {
									throw_error!(a.span(), "no additional arguments were expected");
								}
								
								result
							},
							Some(TokenTree::Literal(ref lit)) => {
								span = lit.span();
								let mut string = lit.to_string();
								
								if is_ease_comparison {
									throw_error!(span, "the macro was not intended to compare only string data, use 'if' better");
								}
								
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
								
								
								unsafe { MacroString::literal(string) }
							},
							Some(ref a) => throw_error!(a.span(), "unsupported input argument type for condition, expected '$a' or 'String'"),
							_ =>  throw_error!(span, "empty condition record, expected '$a == $b'"),
						};
						
						
						
						args_section.push((reverse_data, arg0, arg1).into());
						//end.
						
						*self.buffer = iter.next();
						match *self.buffer {
							None => return t(self, condition_seq_blocks, span), //OK, END IF!
							
							Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
								('&', Spacing::Joint) => {
									span = punct.span();
									*self.buffer = iter.next();
									
									match *self.buffer {
										Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
											('&', Spacing::Alone) => {
												span = punct.span();
												
												continue 'decode_conditions;
											},
											(ch, _a) => throw_error!(punct.span(), format!("undefined condition record `{}`. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record.", ch)),
										},
										Some(ref a) => throw_error!(a.span(), "undefined condition record. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record."),
										
										_ => throw_error!(span, "undefined condition record. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record."),
									}
								},
								('|', Spacing::Joint) => {
									span = punct.span();
									*self.buffer = iter.next();
									
									match *self.buffer {
										Some(TokenTree::Punct(ref punct)) => match (punct.as_char(), punct.spacing()) {
											('|', Spacing::Alone) => {
												args_section = condition_seq_blocks.new_seq_blocs();
												
												span = punct.span();
												
												continue 'decode_conditions;
											},
											(ch, _a) => throw_error!(span, format!("undefined condition record `{}`. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record.", ch)),
										},
										Some(ref a) => throw_error!(a.span(), "undefined condition record. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record."),
										
										_ => throw_error!(span, "undefined condition record. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record."),
									}
								},
								(ch, _a) => throw_error!(punct.span(), format!("undefined condition record `{}`. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record.", ch)),
							},
							Some(ref a) => throw_error!(a.span(), "undefined condition record. expected conditional statements 'and' ('&&') or conditional 'or' ('||'), or empty record."),
						}
					];
				},
				Some(ref a) => throw_error!(a.span(), "undefined condition record form, expected '($a == $i)'"),
				_ => throw_error!(span, "the entry was interrupted, the description of the condition '($ a == $ i)' was expected"),
			}
		];
		unimplemented!();
		
	}
	
	#[inline]
	pub fn expect_iforelse<T: FnMut(Self, (TokenStream, Option<TokenStream>), Span) -> TokenStream>(mut self, mut t: T, mut span: Span) -> TokenStream {
		self.buffer.next();
		
		let ok_stream = match *self.buffer {
			Some(TokenTree::Group(ref mut group)) => {
				span = group.span();
				group.stream()
			},
			Some(ref a) => throw_error!(a.span(), "expected code block"),
			_ => throw_error!(span, "expected code block"),
		};

		let err_stream = {
			self.buffer.next();
			match *self.buffer {
				Some(TokenTree::Ident(ref ident)) => match ident.to_string().as_str() {
					"else" => {
						span = ident.span();
						self.buffer.next();
						
						match *self.buffer {
							Some(TokenTree::Group(ref group)) => {
								span = group.span();
								Some(group.stream())
							},
							Some(ref a) => throw_error!(a.span(), "expected code block"),
							None => throw_error!(span, "expected code block"),
						}
					},
					
					a => throw_error!(ident.span(), format!("undefined record `{}`, expected 'else' (description of the condition for negation)", a)),
				},
				Some(ref a) => throw_error!(a.span(), "undefined record , expected 'else' (description of the condition for negation)"),
				None => None, //ok
			}
		};
		
		t(self, (ok_stream, err_stream), span)
	}
}


#[inline]
pub (crate) fn new_decode_tt_ident_token(token: TokenStream) -> DecodeTTIFIdentToken<impl Iterator<Item = TokenTree>> {
	let iter = token.into_iter();
	
	DecodeTTIFIdentToken::__new(RecursionBuffer::new(iter))
}



pub fn tt_ident(input: TokenStream) -> TokenStream {
	let decode = new_decode_tt_ident_token(input);
	
	decode.expect_ident("if", |decode, str_function, span| {
		match str_function.as_str() {
			"if" => {
				drop(str_function);
				
				let mut vec: Vec<Vec<UniversalArgBlock<_, _>>> = Vec::with_capacity(6);
				decode.decode_arguments(&mut vec, |decode, vec, span| {
					decode.expect_iforelse(|_decode, (true_token, false_token), span| {
						match vec.is_equality() {
							EqSeqBlocks::TrueEq => true_token,
							EqSeqBlocks::FalseEq => match false_token {
								Some(a) => a,
								_ => crate::token_gen::empty_token(),
							},
							EqSeqBlocks::EmptyArray => throw_error!(span, "the entry was interrupted, the description of the condition '($ a == $ i)' was expected"),
						}
					}, span)
				}, span)
			},
			a => throw_error!(span, format!("unknown input `{}`, expected 'if'", a)),
		}
	}, Span::call_site())
}

