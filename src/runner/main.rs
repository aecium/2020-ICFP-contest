use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

struct Line {
    num: usize,
    key: String,
    src: String,
    symbols: Vec<Symbol>,
    value: Vec<Symbol>,
    has_value: bool,
}

fn main() {
    let mut lines: BTreeMap<String, Line> = BTreeMap::new();

    if let Ok(file_lines) = read_lines("galaxy.txt") {
        let mut i = 0;

        for line in file_lines {
            i+=1;
            if let Ok(l) = line {
                println!("Line {}: {}", i, l);
                let mut sym_iter = l.split_whitespace();
                let key = sym_iter.next().expect("Oops, no variable name.");
                let _eq = sym_iter.next().expect("Oops, no equals.");
                let mut symbols: Vec<Symbol> = Vec::new();

                print!("{} = ", key);
                for s in sym_iter {
                    let num = s.parse::<i64>();
                    if num.is_ok() {
                        let mut v = Vec::new();
                        v.push(num.unwrap());
                        symbols.push(Symbol::Value(v));
                    } else if s.eq("ap") {
                        continue;
                    } else if s.starts_with(":") {
                        symbols.push(Symbol::Variable(s.to_string()));
                    } else if s.eq("cons") {
                        symbols.push(Symbol::Cons);
                    } else if s.eq("neg") {
                        symbols.push(Symbol::Negate);
                    } else if s.eq("nil") {
                        symbols.push(Symbol::Value(Vec::new()));
                    } else {
                        println!();
                        println!("Line {}: Error parsing symbol.", i);
                        println!("TODO: {}", s);
                        //panic!("At the disco!");
                    }
                    print!("{},", s);
                }
                println!();

                lines.insert(key.to_string(), Line {
                    num: i,
                    key: key.to_string(),
                    src: l.to_string(),
                    symbols: symbols,
                    value: Vec::new(),
                    has_value: false,
                });
            }
            if i > 200 {
                break;
            }
        }

        let keys: Vec<String> = lines.keys().cloned().collect();
        for key in keys {
            let symbols = lines.get(&key).expect("Unpossible.").symbols.clone();
            let (value, _remaining) = eval(&mut lines, &symbols);
            let line = lines.get_mut(&key).expect("Unpossible.");
            println!("key: {} - val: {:?}", line.key, value);
            line.value = value;
        }

        println!();
        println!("Done!  Values:");
        for var in lines.values() {
            println!("src: {}\n val: {:?}\n", var.src, var.value);
        }
    }
}

fn eval(vars: &mut BTreeMap<String, Line>, symbols: &[Symbol]) -> (Vec<Symbol>, Vec<Symbol>) {
    let mut value: Vec<Symbol> = Vec::new();
    let mut symbols = symbols.to_vec();
    while symbols.len() > 0 {
        let s = symbols.remove(0);
        match s.clone() {
            Symbol::Value(_val) => {
                value.push(s);
                //value = val.to_vec();
                break;
            },
            Symbol::Cons => {
                let (op1, remaining) = eval(vars, &symbols);
                let (op2, remaining) = eval(vars, &remaining);
                let mut ops: Vec<Symbol> = Vec::new();
                ops.extend(op1);
                ops.extend(op2);
                let mut values: Vec<i64> = Vec::new();
                for op in ops {
                    match op {
                        Symbol::Value(vec) => {
                            values.extend(vec);
                        },
                        _ => {
                            panic!("Awe, snap!");
                        }
                    }
                    
                }
                symbols = remaining.to_vec();
                symbols.push(Symbol::Value(values));
            },
            Symbol::Negate => {
                let (op1, remaining) = eval(vars, &symbols);
                let mut ops: Vec<Symbol> = op1.to_vec();
                let mut values: Vec<i64> = Vec::new();
                /*let val1 = ops[0].clone();
                match val1 {
                    Symbol::Value(vec) => {
                        vec[0] = -vec[0];
                    },
                    _ => {
                        panic!("Awe, snap!");
                    }
                };*/
                for op in ops {
                    match op {
                        Symbol::Value(vec) => {
                            values.extend(vec);
                        },
                        _ => {
                            panic!("Awe, snap!");
                        }
                    }
                    
                }
                symbols = remaining.to_vec();
                symbols.push(Symbol::Value(values));
            },
            Symbol::Variable(key) => {
                let value: Vec<Symbol>;
                println!("variable key: {}", key);
                let mut symbols = vars.get(&key).expect("Oh, no symbols!").symbols.clone();
                let has_value = vars.get(&key).expect("Oh, no has_value!").has_value;
                if !has_value {
                    let (op1, remaining) = eval(vars, &symbols);
                    symbols.extend(op1);
                } else {
                    symbols.extend(vars.get(&key).expect("Oh, no!").value.clone());
                }
            }
            _ => {
                panic!("Please implement {:?}", s);
            }
        };
    }
    (value, symbols)
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone,Debug)]
enum Symbol {
    Value(Vec<i64>),
    Variable(String),
    Cons, //(or Pair)
    Equality,
    Successor,
    Predecessor,
    Sum,
    Product,
    IntegerDivision,
    StrictLessThan,
    Modulate,
    Demodulate,
    SendNow, //Send
    Negate,
    FuncApp, //Function Application
    SC, //S Combinator
    CC, //C Combinator
    BC, //B Combinator
    TC, // True (K Combinator)
    FC, // False
    PowerOfTwo,
    IC, //i Combinator
    Car, //(First)
    Cdr, //(Tail)
    Nil, //(Empty List)
    IsNil, //(Is Empty List)
    LCS, //List Construction Syntax
    Vector,
    Draw,
    Checkerboard,
    MulDraw, //Multiple Draw
    ModList, //Modulate List
    Send0,
    Is0,
    Interact,
    InterProt, //Interaction Protocol
    StatesessDraw, //Stateless Drawing Protocol
    StatefullDraw, //Statefull Drawing Protocol
    Galaxy,
}