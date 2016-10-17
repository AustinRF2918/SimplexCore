mod parsing;
mod expression;
mod atom;

extern crate num;
use num::FromPrimitive;
use num::bigint::BigInt;
use num::bigint::BigUint;

#[macro_use]
extern crate decimal;

#[macro_use]
extern crate lazy_static;

extern crate regex;

fn main() {
    println!("SimplexCore Test Environment Loaded.");
}

