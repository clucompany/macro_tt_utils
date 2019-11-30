

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EqSeqBlocks {
	TrueEq,
	FalseEq,
	EmptyArray,
}

impl EqSeqBlocks {
	#[inline]
	pub const fn true_eq() -> Self {
		EqSeqBlocks::TrueEq
	}
	
	#[inline]
	pub const fn false_eq() -> Self {
		EqSeqBlocks::FalseEq
	}
	
	#[inline]
	pub const fn empty_array() -> Self {
		EqSeqBlocks::EmptyArray
	}
}

impl From<bool> for EqSeqBlocks {
	#[inline]
	fn from(b: bool) -> Self {
		match b {
			true => Self::true_eq(),
			_ => Self::false_eq()
		}
	}
}