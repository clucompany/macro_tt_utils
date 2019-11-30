
use alloc::string::ToString;
use alloc::string::String;
use proc_macro::Ident;
use core::ops::Deref;
use core::fmt::Debug;
use core::mem::ManuallyDrop;


#[derive(Clone)]
pub struct MacroString {
	string: ManuallyDrop<String>,
	real_str: ManuallyDrop<&'static str>,
}


impl Drop for MacroString {
	fn drop(&mut self) {
		unsafe {
			ManuallyDrop::drop(&mut self.real_str);
			ManuallyDrop::drop(&mut self.string);
		}
	}
}

impl ToString for MacroString {
	#[inline(always)]
	fn to_string(&self) -> String {
		self.real_str.to_string()
	}
}

impl Into<String> for MacroString {
	#[inline(always)]
	fn into(self) -> String {
		self.real_str.to_string()
	}
}

impl PartialEq for MacroString {
	#[inline]
	fn eq(&self, r: &Self) -> bool {
		self.real_str == r.real_str
	}
}

impl<'a> PartialEq<&'a str> for MacroString {
	#[inline]
	fn eq(&self, r: &&'a str) -> bool {
		*self.real_str == *r
	}
}

impl Debug for MacroString {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.write_str("MacroString(raw: ")?;
		Debug::fmt(self.string.as_str(), f)?;
		f.write_str(", real: ")?;
		Debug::fmt(self.as_str(), f)?;
		f.write_str(")")
	}
}

impl MacroString {
	#[inline]
	const unsafe fn new(s: String, real_str: &'static str) -> Self {
		Self {
			string: ManuallyDrop::new(s),
			real_str: ManuallyDrop::new(real_str),
		}
	}

	pub fn ident(i: &Ident) -> Self {
		Self::string(i.to_string())
	}
	
	pub unsafe fn literal(s: String) -> Self {
		#[allow(unused_unsafe)]
		let real_str: &'static str = unsafe { core::mem::transmute(s.get_unchecked(1..)) };
		Self::new(s, real_str)
	}
	
	pub fn string(s: String) -> Self {
		let real_str: &'static str = unsafe { core::mem::transmute(s.as_str()) };
		unsafe { Self::new(s, real_str) }
	}
	
	#[inline]
	pub fn as_str<'a>(&'a self) -> &'a str {
		*self.real_str
	}
}

impl AsRef<str> for MacroString {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl Deref for MacroString {
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl From<String> for MacroString {
	#[inline(always)]
	fn from(s: String) -> Self {
		MacroString::string(s)
	}
}

impl From<&Ident> for MacroString {
	#[inline(always)]
	fn from(s: &Ident) -> Self {
		MacroString::ident(s)
	}
}
