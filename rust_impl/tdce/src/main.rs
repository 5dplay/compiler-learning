use std::{collections::{HashMap, HashSet}, io::{self, Read}};
use compiler_tools::form_blocks::{form_blocks};
use serde_json::Value;

fn delete_locally_unused_instruction(func: &mut Value) -> bool {

    let blocks = form_blocks(&func["instrs"]);
    let mut new_instrs = Vec::new();
    let mut to_drop = HashSet::<usize>::new();
    let mut index = 0;
    let mut changed = false;

    
    for block in &blocks {
        let mut last_def = HashMap::<String, usize>::new();
        for instr in block {
            if let Some(args) = instr.get("args") {
                for arg in args.as_array().unwrap() {
                    let arg_str = arg.as_str().unwrap().to_string();
                    last_def.remove(&arg_str);
                }
            }
            if let Some(dest) = instr.get("dest") {
                let var = dest.as_str().unwrap().to_string();
                if let Some(pos) =last_def.get(&var) {
                    to_drop.insert(pos.clone());
                    changed = true;
                }
                last_def.insert(var, index);
            }
            index += 1;
        }
    }

    index = 0;
    for block in &blocks {
        for instr in block {
            if !to_drop.contains(&index) {
                new_instrs.push(instr.clone());
            }
            index += 1;
        }
    }

    func["instrs"] = Value::Array(new_instrs);

    changed

}
fn delete_globally_unused_instruction(func: &mut Value) -> bool {
    //Find all the variables used as arguments to any instruction
    //Note that it isn't a syntax check, so don't think about used before defined
    
    let mut var_used = HashSet::<String>::new();
    let mut changed = false;

    let instrs = &func["instrs"];
    for instr in instrs.as_array().unwrap() {
        if let Some(args) = instr.get("args") {
            for arg in args.as_array().unwrap() {
                //println!("variable {} is used as argument", arg.as_str().unwrap());
                var_used.insert(arg.as_str().unwrap().to_string());
            }
        }
    }

    let mut new_instrs = Vec::new();
    for instr in instrs.as_array().unwrap() {
        if let Some(dest) =  instr.get("dest") {
            let var = dest.as_str().unwrap();
            if var_used.contains(var) {
                new_instrs.push(instr.clone());
            } else {
                changed = true;
                //println!("Drop {} because {} not in set", instr, var);
            }
        } else {
            new_instrs.push(instr.clone());
        }
    }

    func["instrs"] = Value::Array(new_instrs);

    changed
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut bril_json: Value = serde_json::from_str(&buffer).unwrap();

    for func in bril_json["functions"].as_array_mut().unwrap() {
        //println!("before:\n {}", func);
        //continue until no change
        while delete_globally_unused_instruction(func) || delete_locally_unused_instruction(func) {};
        //println!("after:\n {}", func);
    }

    print!("{}", bril_json);
}
