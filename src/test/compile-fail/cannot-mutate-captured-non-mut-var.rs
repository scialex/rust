// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(unboxed_closures)]

fn to_fn_once<A,F:FnOnce<A>>(f: F) -> F { f }

fn main() {
    let x = 1;
    to_fn_once(move|:| { x = 2; });
    //~^ ERROR: cannot assign to immutable captured outer variable

    let s = std::old_io::stdin();
    to_fn_once(move|:| { s.read_to_end(); });
    //~^ ERROR: cannot borrow immutable captured outer variable
}
