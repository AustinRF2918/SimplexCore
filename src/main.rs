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

use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use atom::numbers::number::Numeric;
use parsing::utilities::numerics::representable_numeric;
use regex::Regex;

struct State {
    num_map: HashMap<String, Numeric>,
    current_input: usize
}

impl State {
    pub fn new() -> State{
        State {
            num_map: HashMap::new(),
            current_input: 0
        }
    }
}

fn evaluate(line: String, state: &mut State) {
    match assignment(&line, state) {
        Some((x, y)) => {
            state.num_map.insert(x.clone(), y);
            println!("Out[{}]= {} == {} [Assignment]", state.current_input, x.clone(), y.as_str());
        },
        None => {
            match operator(&line, state) {
                Some(s) => {
                    println!("Out[{}]= {} [Operator]", state.current_input, s.as_str());
                }
                None => {
                        match parse_v_name(&line, state) {
                        Some(x) => {
                            println!("Out[{}]= {} [VName Lookup]", state.current_input, x.as_str());
                        }
                        None => {
                            println!("Out[{}]= {} [Unknown Op]", state.current_input, line);
                        }
                    }
                }
            }

        }
    }

    state.current_input = state.current_input + 1;
}


fn assignment(line: &String, state: &mut State) -> Option<(String, Numeric)> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\s*(?P<lhs>[a-zA-Z]+)\s*=\s*(?P<rhs>[a-zA-Z0-9|.]*)\s*$").unwrap();
    }

    let captures = RE.captures(line.as_str());

    match captures {
        Some(c) => {
            let lhs = c.name("lhs").unwrap();
            let rhs = c.name("rhs").unwrap();

            match Numeric::from_str(rhs).unwrap().to_string().as_str() {
                "NaN" => {
                    println!("{}", rhs.to_string());
                    match operator(&rhs.to_string(), state) {
                        Some(x) => {
                            Some((lhs.to_string(), Numeric::from_str(x.as_str()).unwrap()))
                        }
                        None => {
                            None

                        }
                    }
                }
                _ => {
                    Some((lhs.to_string(), Numeric::from_str(rhs).unwrap()))
                }
            }

        }

        _ => {
            None
        }
    }
}

fn operator(line: &String, state: &mut State) -> Option<String> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\s*(?P<lhs>[a-zA-Z0-9\.]*)\s*(?P<op>[+-/*])\s*(?P<rhs>[a-zA-Z0-9\.]*)\s*$").unwrap();
    }

    let captures = RE.captures(line.as_str());

    match captures {
        Some(c) => {
            let lhs = c.name("lhs").unwrap();
            let rhs = c.name("rhs").unwrap();
            let op = c.name("op").unwrap();

            match op {
                "+" => {
                    match (state.num_map.get(lhs), state.num_map.get(rhs)) {
                        (Some(x), Some(y)) => {
                            Some((*x + *y).to_string())
                        }
                        (Some(x), None) => {
                            let y = Numeric::from_str(rhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((x.to_string() + " + " + rhs).to_string())
                            } else {
                                Some((*x + y).to_string())
                            }
                        }
                        (None, Some(x)) => {
                            let y = Numeric::from_str(lhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((y.to_string() + " + " + lhs).to_string())
                            } else {
                                Some((*x + y).to_string())
                            }
                        }
                        (None, None) => {
                            Some((lhs.to_string() + " + " + rhs))
                        }
                    }
                }
                "-" => {
                    match (state.num_map.get(lhs), state.num_map.get(rhs)) {
                        (Some(x), Some(y)) => {
                            Some((*x - *y).to_string())
                        }
                        (Some(x), None) => {
                            let y = Numeric::from_str(rhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((x.to_string() + " - " + rhs).to_string())
                            } else {
                                Some((*x - y).to_string())
                            }
                        }
                        (None, Some(x)) => {
                            let y = Numeric::from_str(lhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((y.to_string() + " - " + lhs).to_string())
                            } else {
                                Some((*x - y).to_string())
                            }
                        }
                        (None, None) => {
                            Some((Numeric::from_str(lhs).unwrap() - Numeric::from_str(rhs).unwrap()).to_string())
                        }
                    }
                }

                "/" => {
                    match (state.num_map.get(lhs), state.num_map.get(rhs)) {
                        (Some(x), Some(y)) => {
                            Some((*x / *y).to_string())
                        }
                        (Some(x), None) => {
                            let y = Numeric::from_str(rhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((x.to_string() + " / " + rhs).to_string())
                            } else {
                                Some((*x / y).to_string())
                            }
                        }
                        (None, Some(x)) => {
                            let y = Numeric::from_str(lhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((y.to_string() + " / " + lhs).to_string())
                            } else {
                                Some((*x / y).to_string())
                            }
                        }
                        (None, None) => {
                            Some((Numeric::from_str(lhs).unwrap() / Numeric::from_str(rhs).unwrap()).to_string())
                        }
                    }
                }

                "*" => {
                    match (state.num_map.get(lhs), state.num_map.get(rhs)) {
                        (Some(x), Some(y)) => {
                            Some((*x * *y).to_string())
                        }
                        (Some(x), None) => {
                            let y = Numeric::from_str(rhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((x.to_string() + " * " + rhs).to_string())
                            } else {
                                Some((*x * y).to_string())
                            }
                        }
                        (None, Some(x)) => {
                            let y = Numeric::from_str(lhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((y.to_string() + " * " + lhs).to_string())
                            } else {
                                Some((*x * y).to_string())
                            }
                        }
                        (None, None) => {
                            Some((Numeric::from_str(lhs).unwrap() * Numeric::from_str(rhs).unwrap()).to_string())
                        }
                    }
                }

                "==" => {
                    match (state.num_map.get(lhs), state.num_map.get(rhs)) {
                        (Some(x), Some(y)) => {
                            Some((*x == *y).to_string())
                        }
                        (Some(x), None) => {
                            let y = Numeric::from_str(rhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((x.to_string() == rhs).to_string())
                            } else {
                                Some((*x == y).to_string())
                            }
                        }
                        (None, Some(x)) => {
                            let y = Numeric::from_str(lhs).unwrap();
                            if y.simplify() == Numeric::NaN {
                                Some((y.to_string() == lhs).to_string())
                            } else {
                                Some((*x == y).to_string())
                            }
                        }
                        (None, None) => {
                            Some((Numeric::from_str(lhs).unwrap() == Numeric::from_str(rhs).unwrap()).to_string())
                        }
                    }
                }
                _ => {
                    None
                }
            }
        }

        _ => {
            None
        }
    }
}

fn parse_v_name(line: &String, state: &mut State) -> Option<Numeric> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\s*(?P<data>[a-zA-Z]+)\s*$").unwrap();
    }

    let captures = RE.captures(line.as_str());

    match captures {
        Some(c) => {
            match c.name("data") {
                Some(x) => {
                    match state.num_map.get(x) {
                        Some(y) => {
                            Some(y.to_owned())
                        }
                        None => {
                            None
                        }
                    }
                }
                None => {
                    None
                }
            }
        }

        _ => {
            None
        }
    }
}

fn main() {
    println!("SimplexCore Test Environment Loaded.");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut program_state = State::new();

    let mut line_num = 0;

    loop {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        evaluate(line, &mut program_state);
        line_num = line_num + 1;
    }
}

