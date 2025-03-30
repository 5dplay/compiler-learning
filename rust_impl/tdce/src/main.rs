use std::{collections::HashSet, io::{self, Read}};
use serde_json::Value;

fn delete_globally_unused_instruction(func: &mut Value) {
    //Find all the variables used as arguments to any instruction
    //Note that it isn't a syntax check, so don't think about used before defined
    
    let mut var_used = HashSet::<String>::new();

    let instrs = &func["instrs"];
    for instr in instrs.as_array().unwrap() {
        match instr.get("args") {
            Some(args) => {
                for arg in args.as_array().unwrap() {
                    println!("variable {} is used as argument", arg.as_str().unwrap());
                }
            },
            _ => {}
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut bril_json: Value = serde_json::from_str(&buffer).unwrap();
    let mut bril_out = serde_json::json!({});

    for func in bril_json["functions"].as_array_mut().unwrap() {
        delete_globally_unused_instruction(func);
    }
}
