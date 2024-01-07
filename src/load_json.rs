// This module build the physical equations and constant from data defined in json files.
use serde::Deserialize;
use std::collections::HashMap;

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
    // Iterate over each equation in the JSON
    json.equations
        .iter()
        .map(|(equation_name, equation_details)| {
            // For each equation, iterate over its variables
            let formatted_variables = equation_details
                .variables
                .iter()
                // Format each variable as "variable_name: f64"
                .map(|(variable_name, _)| format!("{}: {}", variable_name, "f64"))
                // Collect the formatted variables into a vector
                .collect::<Vec<_>>()
                // Join the formatted variables into a string, separated by commas
                .join(", ");

            let additional_info = format!(
                "/*\n\tDesc: {}\n{}\n*/",
                equation_details.description,
                equation_details
                    .variables
                    .iter()
                    .map(|(name, desc)| format!("\t{}: {}", name, desc))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            // Add the comment to rust function
            let rust_function = format!(
                "{}\nfn {}({}) -> f64 {{\n\t{}\n}}\n",
                additional_info, equation_name, formatted_variables, equation_details.result
            );

            // Return a tuple of the equation name and the Rust function definition
            (equation_name.to_string(), rust_function)
            // Collect the tuples into a HashMap
        })
        .collect()
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
