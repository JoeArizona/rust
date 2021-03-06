// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z continue-parse-after-error

// Test that the parser is error correcting missing idents. Despite a parsing
// error (or two), we still run type checking (and don't get extra errors there).

fn main() {
    let y = 42;
    let x = y.;  //~ ERROR unexpected token
    let x = y.();  //~ ERROR unexpected token
    let x = y.foo; //~ ERROR no field
}
