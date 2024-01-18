//  This module defines all physical constants used in this program
use lazy_static::lazy_static;
use load_json::{get_equations, get_physical_consts, read_json, ClassicalPhysicsJson};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

pub fn construct_equation_file(equations: &HashMap<String, String>) {
    let filename: &str = "equations.rs";
    let mut path = PathBuf::from("src");
    path.push(filename);

    println!("Constructing Equation File from json data: {}", filename);
    println!("Verify file does not exist: {}", filename);
    // check if file exists, if yes then overwrite it
    if path.exists() {
        println!("File {} exists, start overwriting...", filename);
        let _ = fs::remove_file(&path);
    }

    let mut file = fs::File::create(&path).unwrap();
    // Loop through all k,v in equations and write to file with \n
    for (_, value) in equations {
        file.write_all(value.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.flush().unwrap(); // Ensure all writes are completed
    println!("Successfully constructed equation file!");
}
