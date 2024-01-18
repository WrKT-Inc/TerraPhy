extern crate lazy_static;
extern crate serde;
mod load_json;
mod constants;

use constants::{EQUATIONS, construct_equation_file};

fn main() {
    construct_equation_file(&EQUATIONS);
}