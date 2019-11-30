
use alloc::string::String;
use proc_macro::Diagnostic;
use proc_macro::Span;

pub trait SpanElement {
	fn as_span(&self) -> &Span;
	fn error<T: Into<String>>(&self, message: T) -> Diagnostic;
	fn warning<T: Into<String>>(&self, message: T) -> Diagnostic;
	fn note<T: Into<String>>(&self, message: T) -> Diagnostic;
	fn help<T: Into<String>>(&self, message: T) -> Diagnostic;
}

impl SpanElement for Span {
	#[inline(always)]
	fn as_span(&self) -> &Span {
		self
	}
	
	#[inline(always)]
	fn error<T: Into<String>>(&self, message: T) -> Diagnostic {
		Span::error(*self, message)
	}
	
	#[inline(always)]
	fn warning<T: Into<String>>(&self, message: T) -> Diagnostic {
		Span::warning(*self, message)
	}
	
	#[inline(always)]
	fn note<T: Into<String>>(&self, message: T) -> Diagnostic {
		Span::note(*self, message)
	}
	
	#[inline(always)]
	fn help<T: Into<String>>(&self, message: T) -> Diagnostic {
		Span::help(*self, message)
	}
}
/*
impl<'a, A> SpanElement for &'a A where A: SpanElement {
	#[inline(always)]
	fn as_span(&self) -> &Span {
		A::as_span(self)
	}
	
	#[inline(always)]
	fn error<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::error(self, message)
	}
	
	#[inline(always)]
	fn warning<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::warning(self, message)
	}
	
	#[inline(always)]
	fn note<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::note(self, message)
	}
	
	#[inline(always)]
	fn help<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::help(self, message)
	}
}*/

impl<'a, A> SpanElement for &'a mut A where A: SpanElement {
	#[inline(always)]
	fn as_span(&self) -> &Span {
		A::as_span(self)
	}
	
	#[inline(always)]
	fn error<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::error(self, message)
	}
	
	#[inline(always)]
	fn warning<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::warning(self, message)
	}
	
	#[inline(always)]
	fn note<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::note(self, message)
	}
	
	#[inline(always)]
	fn help<T: Into<String>>(&self, message: T) -> Diagnostic {
		A::help(self, message)
	}
}


