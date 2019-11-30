

use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct ReverseEqBlock {
	is_reverse: bool
}
impl Eq for ReverseEqBlock {}

impl PartialEq<bool> for ReverseEqBlock {
	#[inline]
	fn eq(&self, n: &bool) -> bool {
		self.is_reverse == *n
	}
}

impl PartialEq<ReverseEqBlock> for bool {
	#[inline]
	fn eq(&self, n: &ReverseEqBlock) -> bool {
		*self == n.is_reverse
	}
}

impl Debug for ReverseEqBlock {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		let is_str = match self.is_reverse {
			true => "Enabled",
			_ => "Disabled",
		};
		
		f.write_str(is_str)
	}
}

impl Default for ReverseEqBlock {
	#[inline(always)]
	fn default() -> Self {
		Self::disable()
	}
}

impl ReverseEqBlock {
	#[inline]
	const fn new(b: bool) -> Self {
		Self {
			is_reverse: b
		}
	}
	
	#[inline]
	pub const fn disable() -> Self {
		Self::new(false)
	}
	
	#[inline]
	pub fn next(&mut self) {
		self.is_reverse = match self.is_reverse {
			true => false,
			_ => true,
		}
	}
	
	#[inline]
	pub fn to_one(&mut self) {
		self.is_reverse = match self.is_reverse {
			false => true,
			_ => unimplemented!(),
		}
	}
	
	#[inline]
	pub fn reverse(&self, b: &mut bool) {
		match self.is_reverse {
			true => *b = !*b,
			_ => {},
		}
	}
}

/*
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ReverseEqBlock {
	Disable,
	One,
	TwoDisable,
}

impl Default for ReverseEqBlock {
	#[inline(always)]
	fn default() -> Self {
		Self::disable()
	}
}

impl ReverseEqBlock {
	#[inline]
	pub const fn disable() -> Self {
		Self::Disable
	}
	
	#[inline]
	pub const fn one() -> Self {
		Self::One
	}
	
	#[inline]
	pub const fn two() -> Self {
		Self::TwoDisable
	}
	
	#[inline]
	pub fn next(&mut self) {
		*self = match self {
			Self::Disable => Self::One,
			Self::One => Self::TwoDisable,
			Self::TwoDisable => unimplemented!(),
		}
	}
	
	#[inline]
	pub fn to_one(&mut self) {
		*self = match self {
			Self::Disable => Self::One,
			_ => unimplemented!(),
		}
	}
	
	#[inline]
	pub fn reverse(&self, b: &mut bool) {
		match self {
			Self::Disable => {},
			Self::One => *b = !*b,
			Self::TwoDisable => {},
		}
	}
}*/

/*
type FnEqBlock = fn(&mut bool);

#[derive(Clone)]
pub struct ReverseEqBlock(FnEqBlock);

impl Debug for ReverseEqBlock {
	#[inline]
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		let debug_str = if self.0 as usize == r_off as usize {
			"r_one"
		}else 
		
		if self.0 as usize == r_one as usize {
			"r_two"
		}else {
			"unimplemented"
		};
		
		write!(fmt, "ReverseEqBlock({:?}, {})", self.0 as usize, debug_str)
	}
}

impl Deref for ReverseEqBlock {
	type Target = FnEqBlock;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}


#[inline]
fn r_off(_b: &mut bool) {}

#[inline]
fn r_one(b: &mut bool) {*b = !*b;}

#[inline]
fn r_two(_b: &mut bool) {}

impl ReverseEqBlock {
	#[inline]
	fn new(r: FnEqBlock) -> Self {
		ReverseEqBlock(r)
	}
	
	#[inline(always)]
	pub fn default() -> Self {
		Self::new(r_off)
	}
	
	#[inline(always)]
	pub fn one() -> Self {
		Self::new(r_one)
	}
	
	#[inline(always)]
	pub fn two() -> Self {
		Self::new(r_two)
	}
	
	#[inline]
	pub fn next(&mut self) {
		self.0 = if self.0 as usize == r_off as usize {
			r_one as _
		}else
		
		if self.0 as usize == r_one as usize {
			r_two as _
		}else {
			unimplemented!();
		}
	}
	
	#[inline]
	pub fn to_one(&mut self) {
		self.0 = if self.0 as usize  == r_off as usize {
			r_one as _
		}else {
			unimplemented!();
		}
	}
}
*/