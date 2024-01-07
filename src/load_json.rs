// This module build the physical equations and constant from data defined in json files.
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ClassicalPhysicsJson {
    name: String,
    description: String,
    constants: HashMap<String, f64>,
    equations: HashMap<String, Equation>,
}

#[derive(Deserialize, Debug)]
struct Equation {
    description: String,
    variables: HashMap<String, String>,
    result: String, // This represent a programable equations in string format.
}

pub fn read_json(filename: &str) -> Result<ClassicalPhysicsJson, Box<dyn std::error::Error>> {
    let json_str = std::fs::read_to_string(filename)?;
    let json: ClassicalPhysicsJson = serde_json::from_str(&json_str)?;
    Ok(json)
}

pub fn get_physical_consts(json: &ClassicalPhysicsJson) -> HashMap<String, f64> {
    let mut physical_consts = HashMap::new();
    for (key, value) in &json.constants {
        physical_consts.insert(key.to_string(), *value);
    }
    physical_consts
}

// build up a simple rust function from mathematical equation
pub fn get_equations(json: &ClassicalPhysicsJson) -> HashMap<String, String> {
    json.equations.iter().map(|(key, value)| {
        let vars = value.variables.iter()
            .map(|(var, _)| format!("{}: {}", var, "f64"))
            .collect::<Vec<_>>()
            .join(", ");
        let rust_func = format!("fn {}({}) -> f64 {{{}}}", key, vars, value.result);
        (key.to_string(), rust_func)
    }).collect()
}

fn main() {
    match read_json("classical_physics.json") {
        Ok(json) => {
            println!("CONSTANTS: {:?}\n\n", get_physical_consts(&json));
            println!("EQUATIONS: {:?}\n\n", get_equations(&json));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
        
    }
}