// Copyright 2014 James R. Garrison
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ARPACK wrapper
//!

#![crate_name = "arpack"]
#![unstable]
//#![license = "MIT/ASL2"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(collections)]

pub mod symmetric;
pub mod nonsymmetric;
