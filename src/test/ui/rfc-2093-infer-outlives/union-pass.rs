// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-pass

#![feature(infer_outlives_requirements)]
#![feature(untagged_unions)]
#![allow(unions_with_drop_fields)]

// Type T needs to outlive lifetime 'a. This is not reported due to
// a compilation error in Bar.
union Foo<'a, T> {
    field1: Bar<'a, T>
}

// Type U needs to outlive lifetime 'b
union Bar<'b, U> {
    field2: &'b U
}


// Type K needs to outlive lifetime 'c.
union Ying<'c, K> {
    field1: &'c Yang<K>
}

union Yang<V> {
    field2: V
}

fn main() {}

