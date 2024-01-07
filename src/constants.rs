//  This module defines all physical constants used in this program
#[macro_use]
extern crate lazy_static;

mod load_json;

use load_json::{read_json, get_physical_consts, ClassicalPhysicsJson, get_equations};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

const JSON_FILENAME: &str = "classical_physics.json";

lazy_static! {
    static ref JSON_DATA: ClassicalPhysicsJson = read_json(JSON_FILENAME).unwrap();
}

lazy_static! {
    pub static ref PHYSICAL_CONSTS: HashMap<String, f64> = get_physical_consts(&JSON_DATA);
}

lazy_static! {
    pub static ref EQUATIONS: HashMap<String, String> = get_equations(&JSON_DATA);
}

fn construct_equation_file(equations: &HashMap<String, String>) -> () {
    let filename: &str = "equations.rs";
    
    println!("Constructing Equation File from json data: {}", filename);
    println!("Verify file does not exist: {}", filename);
    // check if file exists, if yes then overwrite it
    if fs::metadata(filename).is_ok() {
        println!("File {} exists, start overwriting...", filename);
        fs::remove_file(filename).unwrap();
    }

    let mut file = fs::File::create(filename).unwrap();
    // Loop through all k,v in equations and write to file with \n
    for (_, value) in equations {
        file.write_all(value.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    println!("Successfully constructed equation file!");
}

fn main() {
    println!("Testing load json data from file: {}\n", JSON_FILENAME);
    println!("CONSTANTS: {:?}\n\n", *PHYSICAL_CONSTS);
    println!("EQUATIONS: {:?}\n\n", *EQUATIONS);
    construct_equation_file(&*EQUATIONS);

}