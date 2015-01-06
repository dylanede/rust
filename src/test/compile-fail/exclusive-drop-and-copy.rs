// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// issue #20126

#[derive(Copy)]
struct Foo;

impl Drop for Foo {
    //~^ ERROR the `Drop` trait may not be implemented on a type that implements `Copy`
    fn drop(&mut self) {}
}

fn main() {}
