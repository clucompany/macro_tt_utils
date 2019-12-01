#![feature(proc_macro_diagnostic)]
#![feature(const_fn)]
#![feature(proc_macro_span)]
#![feature(proc_macro_mixed_site)]

#![no_std]

//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

// #Ulin Project 1819

/*!
Procedural Macro Utilities (tt_ident, throw_diagnostics, deref_span).

// Experimental API

## tt_ident
Manipulation of input names in macros, currently only name comparison is possible.

```rust
#![feature(proc_macro_hygiene)]

use macro_tt_utils::tt_ident;
use macro_tt_utils::throw_compile_warning;
use macro_tt_utils::throw_compile_error;


#[macro_export]
macro_rules! macros_generate {
	( fn name_checker($i:ident) -> | $i2:ident | $b:block ) => {
		throw_compile_warning!(@root "We are testing warning.");
		tt_ident! {
			if ($i == $i2 || $i == "__hidden_a" || $i2 == "__hidden_a") {
				throw_compile_error!(@root "Perhaps undefined behavior");
			}
		}
	}
}

fn main() {
	macros_generate! {
		fn name_checker(a) -> |a| {
			ff
		}
	}
}


/* 
warning: We are testing warning.
--> src/main.rs:21:2
|
21 |       macros_generate! {
|  _____^
22 | |         fn name_checker(a) -> |a| {
23 | |             ff
24 | |         }
25 | |     }
| |_____^

error: Perhaps undefined behavior
--> src/main.rs:21:2
|
21 |       macros_generate! {
|  _____^
22 | |         fn name_checker(a) -> |a| {
23 | |             ff
24 | |         }
25 | |     }
| |_____^
*/
```

## throw_diagnostics 
Macros to throw compiler errors or warnings (```throw_compile_error```, ```throw_compile_warning```, ```throw_compile_note```, ```throw_compile_help```).

```rust
#![feature(proc_macro_hygiene)]

use macro_tt_utils::throw_compile_warning;

#[macro_export]
macro_rules! macros_generate {
	{ fn name_checker($i:ident) -> | $i2:ident | $b:block } => {
		throw_compile_warning!("#1 We are testing warning.");
		throw_compile_warning!(@root "#2 We are testing warning.");
	}
}

fn main() {
	macros_generate! {
		fn name_checker(a) -> |a| {}
	}
}

/*
warning: #1 We are testing warning.
  --> src/main.rs:8:3
   |
8  |           throw_compile_warning!("#1 We are testing warning.");
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
14 |       macros_generate! {
   |  _____-
15 | |         fn name_checker(a) -> |a| {}
16 | |     }
   | |_____- in this macro invocation

warning: #2 We are testing warning.
  --> src/main.rs:14:2
   |
14 |       macros_generate! {
   |  _____^
15 | |         fn name_checker(a) -> |a| {}
16 | |     }
   | |_____^

*/
```

## deref_span
Unpacks the current span.


```
#![feature(proc_macro_hygiene)]

use macro_tt_utils::deref_span;

#[macro_export]
macro_rules! macros_generate {
	{ fn name_checker($i:ident) -> | $i2:ident | $b:block } => {
		compile_error!("#1 Test");
		
		deref_span! {
			compile_error!("#2 Test");
		}
	}
}

fn main() {
	macros_generate! {
		fn name_checker(a) -> |a| {}
	}
}

/*
error: #1 Test
  --> src/main.rs:8:3
   |
8  |           compile_error!("#1 Test");
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
17 |       macros_generate! {
   |  _____-
18 | |         fn name_checker(a) -> |a| {}
19 | |     }
   | |_____- in this macro invocation

error: #2 Test
  --> src/main.rs:17:2
   |
17 |       macros_generate! {
   |  _____^
18 | |         fn name_checker(a) -> |a| {}
19 | |     }
   | |_____^

error: aborting due to 2 previous errors
*/
```




*/

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


