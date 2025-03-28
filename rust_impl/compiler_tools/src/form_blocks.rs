use serde_json::Value;

pub fn linear_print(json: &str) {
    let mut i = 0;
    let bril_json: Value = serde_json::from_str(json).unwrap();

    for function in bril_json["functions"].as_array().unwrap() {
        println!("function {}:", i);

        let instrs = &function["instrs"];
        let mut j = 0;
        for instr in instrs.as_array().unwrap() {
            println!("instr[{}] = {}", j, instr);
            j += 1;
        }

        i += 1;
    }
}

pub fn form_blocks(instrs: &Value) -> Vec<Vec<Value>> {
    let mut blocks: Vec<Vec<Value>> = Vec::new();
    let mut block: Vec<Value> = Vec::new();
    for instr in instrs.as_array().unwrap() {

        if instr.get("op").is_some() {
            block.push(instr.clone());
            let op = &instr["op"];
            match op {
                Value::String(op_str) => {
                    match op_str.as_str() {
                        "br" |  "jmp" | "ret" => {
                            blocks.push(block.clone());
                            block.clear();
                        }
                        _ => {}
                    }

                },
                _ => {}
            }
        } else {
            //label
            if !block.is_empty() {
                blocks.push(block.clone());
                block.clear();
            }

            block.push(instr.clone());
        }
    }

    if !block.is_empty() {
        blocks.push(block.clone());
    }

    blocks
}

pub fn parse_once(function: &Value) {
    form_blocks(&function["instrs"]);
}

pub fn blocks_print(json: &str) {
    let mut i = 0;
    let bril_json: Value = serde_json::from_str(json).unwrap();

    for function in bril_json["functions"].as_array().unwrap() {
        println!("function {}:", i);

        let instrs = &function["instrs"];
        let mut j = 0;
        let blocks = form_blocks(instrs);
        for block in &blocks {
            println!("block {}", j);
            
            for instr in block {
                println!("\t{}", instr);
            }

            j += 1;
        }

        i += 1;
    }
}
