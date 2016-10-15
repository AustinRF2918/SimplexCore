mod parsing;
mod expression;
mod atom;

extern crate num;
use num::FromPrimitive;
use num::bigint::BigInt;
use num::bigint::BigUint;

#[macro_use]
extern crate decimal;

fn main() {
    println!("SimplexCore Test Environment Loaded.");
}

