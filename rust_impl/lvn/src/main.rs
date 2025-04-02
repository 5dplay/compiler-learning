use std::{collections::{HashMap, HashSet}, io::{self, Read}};
use compiler_tools::form_blocks::{form_blocks};
use serde_json::Value;

enum LvnTupleOp {
    Normal(String, Vec<usize>),
    ExternalVar(String),
}

struct LvnTuple {
    op: LvnTupleOp,
    var: String,
}

fn ParseInstr(instr: &Value, var2num: &mut HashMap<String, usize>, table: &mut Vec<LvnTuple>, output: &mut Vec<Value>) {

    let op_str = instr.get("op").unwrap().as_str().unwrap();

    match op_str {
        "call" => {
            //TODO:skip call instruction with some reason I don't understand
            output.push(instr.clone());
        },
        _  => {
            let op = op_str.to_string();
            let mut vec = Vec::<usize>::new();
            if let Some(dest) = instr.get("dest") {

                if let Some(args) = instr.get("args") {

                    for arg in args.as_array().unwrap() {
                        let mut pos = 0;
                        let arg_str = arg.as_str().unwrap().to_string();
                        
                        // if contains this arg, then substitute it with index
                        // if doesn't contain this arg, then it is from outside block
                        //     First, create new tuple. Then insert to table. Finally, convert name to
                        //     index
                        if let Some(index) = var2num.get(&arg_str) {
                            pos = index.clone();
                        } else {
                            let ext_var = LvnTuple {
                                op: LvnTupleOp::ExternalVar(arg_str.clone()),
                                var: arg_str.clone()
                            };
                            pos = table.len();
                            table.push(ext_var);
                            var2num.insert(arg_str, pos);

                        }
                        vec.push(pos);
                    }

                    //find the same expr, if found, change instr to id instr
                    //if not found, marked as new tuple
                    for t in table {
                        match &t.op {
                            LvnTupleOp::Normal(t_op, t_args_idx) => {
                                if t_op.clone() == op {
                                    continue;
                                }
                                if vec.len() != t_args_idx.len() {
                                    continue;
                                }
                                let mut i = 0;
                                for &idx in t_args_idx {
                                    if vec[i] != idx {
                                        continue;
                                    }
                                    i += 1;
                                }
                            },
                            LvnTupleOp::ExternalVar(var) => {
                            }
                        }
                    }
                }
            } else {
                output.push(instr.clone());
            }
        },
    }
}

//local optimize, can't do it globally, think about diamond flow c <- a + b, where a and b might be
//redefined serveral times
fn lvn(func: &mut Value) {

    let blocks = form_blocks(&func["instrs"]);
    let mut new_instrs = Vec::new();

    
    for block in &blocks {
        let mut var2num = HashMap::<String, usize>::new();
        let mut table = Vec::<LvnTuple>::new();
        for instr in block {
        }
    }

    func["instrs"] = Value::Array(new_instrs);


}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut bril_json: Value = serde_json::from_str(&buffer).unwrap();

    for func in bril_json["functions"].as_array_mut().unwrap() {
        //println!("before:\n {}", func);
        lvn(func);
        //println!("after:\n {}", func);
    }

    print!("{}", bril_json);
}
