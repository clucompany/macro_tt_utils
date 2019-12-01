# macro_tt_utils
Procedural Macro Utilities (```tt_ident```, ```throw_diagnostics```, ```deref_span```).

[![Build Status](https://travis-ci.org/clucompany/Goto.svg?branch=master)](https://travis-ci.org/clucompany/macro_tt_utils)
[![Mit/Apache licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/goto)](https://crates.io/crates/macro_tt_utils)
[![Documentation](https://docs.rs/goto/badge.svg)](https://docs.rs/macro_tt_utils)

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


```rust
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


# License

Copyright 2019 #UlinProject (Denis Kotlyarov) Денис Котляров

Licensed under the MIT License

Licensed under the Apache License, Version 2.0

