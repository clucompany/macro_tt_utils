# macro_tt_utils
Procedural Macro Utilities (tt_ident, throw_diagnostics, deref_span).

[![Build Status](https://travis-ci.org/clucompany/Goto.svg?branch=master)](https://travis-ci.org/clucompany/macro_tt_utils)
[![Mit/Apache licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/goto)](https://crates.io/crates/macro_tt_utils)
[![Documentation](https://docs.rs/goto/badge.svg)](https://docs.rs/macro_tt_utils)


# tt_ident
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
