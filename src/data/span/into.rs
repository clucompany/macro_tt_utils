
use proc_macro::Punct;
use proc_macro::Ident;
use proc_macro::TokenTree;
use proc_macro::Span;

pub trait IntoSpan {
	fn into_span(self) -> Span;
}

impl IntoSpan for Span {
	#[inline]
	fn into_span(self) -> Span {
		self
	}
}

impl<'a> IntoSpan for &'a TokenTree {
	#[inline]
	fn into_span(self) -> Span {
		self.span()
	}
}

impl<'a> IntoSpan for &'a Ident {
	#[inline]
	fn into_span(self) -> Span {
		self.span()
	}
}

impl<'a> IntoSpan for &'a mut Ident {
	#[inline]
	fn into_span(self) -> Span {
		self.span()
	}
}

impl<'a> IntoSpan for &'a Punct {
	#[inline]
	fn into_span(self) -> Span {
		self.span()
	}
}

