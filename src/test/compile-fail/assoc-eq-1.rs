// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test equality constraints on associated types. Check that unsupported syntax
// does not ICE.

#![feature(associated_types)]

pub trait Foo {
    type A;
    fn boo(&self) -> <Self as Foo>::A;
}

fn foo2<I: Foo>(x: I) {
    let _: A = x.boo(); //~ERROR use of undeclared
    let _: I::A = x.boo(); //~ERROR failed to resolve
    //~^ERROR use of undeclared type name `I::A`
}

pub fn main() {}
