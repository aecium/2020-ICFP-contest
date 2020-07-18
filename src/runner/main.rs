use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn read_file_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not load line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
   let filename = &args[1];
    let entrypoint = &args[2];

    let mut state = State::load(read_file_lines(filename));
    state.execute(entrypoint);

    println!();
    println!("Done!");
    /*
    println!("Done!  Values:");
    for i in 0..state.lines.len() {
        println!();
        println!("Line:   {}", i);
        println!("Src:    {}", state.lines[i].src);
        println!("Parsed: {:?}", state.lines[i].parsed);
        println!("Val:    {:?}", state.mem[i]);
    }
    */

    if state.remaining.len() > 0 {
        println!();
        println!("Remaining:");
        println!("{:?}", state.remaining);
    }

    if state.todo.len() > 0 {
        println!();
        println!("Unknown operators:");
        println!("{:?}", state.todo);
    }

    println!();
    println!("Results from evaluating entrypoint {} ({})", entrypoint, state.get_line(entrypoint).src);
    println!("{:?}", state.get_mem(entrypoint));
}

#[derive(Clone)]
struct Line {
    src: String,
    parsed: Vec<Symbol>,
    has_value: bool,
}

struct State {
    keys: HashMap<String, usize>,
    lines: Vec<Line>,
    mem: Vec<Vec<Symbol>>,
    todo: HashMap<String, usize>,
    remaining: Vec<Symbol>,
}

impl State {
    fn get_mem(&self, key: &str) -> Vec<Symbol> {
        self.mem[self.keys.get(key).expect(&format!("Unknown key '{}'.", key)).clone()].clone()
    }

    fn get_line(&self, key: &str) -> Line {
        self.lines[self.keys.get(key).expect(&format!("Unknown key '{}'.", key)).clone()].clone()
    }

    fn load(file_lines: impl IntoIterator<Item = impl Into<String>>) -> State {
        let mut index: HashMap<String, usize> = HashMap::new();
        let mut lines = Vec::new();
        let mut mem = Vec::new();
        let mut todo: HashMap<String, usize> = HashMap::new();
        
        let mut i = 0;
        for file_line in file_lines {
            let l = file_line.into();
            //println!("Line {}: {}", i+1, l);
            let mut sym_iter = l.split_whitespace();
            let key = sym_iter.next().expect("Oops, no variable name.");
            let _eq = sym_iter.next().expect("Oops, no equals.");
            let mut symbols: Vec<Symbol> = Vec::new();

            //print!("{} = ", key);
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
                    //println!();
                    //println!("Line {}: Error parsing symbol.", i);
                    //println!("TODO: {}", s);
                    todo.insert(s.to_string(), 1);
                    //panic!("At the disco!");
                }
                //print!("{},", s);
            }
            //println!();

            index.insert(key.to_string(), i);
            lines.push(Line {
                src: l.to_string(),
                parsed: symbols.clone(),
                has_value: false,
            });
            mem.push(symbols);

            i+=1;
        }


        State {
            keys: index,
            lines: lines,
            mem: mem,
            todo: todo,
            remaining: Vec::new(),
        }
    }

    fn execute(&mut self, entrypoint: &str) {
        let i = self.keys.get(entrypoint).expect("Oh, no!  The entrypoint is missing!").clone();
        let (value, remaining) = self.eval(self.mem[i].clone());
        self.mem[i] = value;
        self.remaining = remaining;
    }

    fn eval(&mut self, symbols: Vec<Symbol>) -> (Vec<Symbol>, Vec<Symbol>) {
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
                    let (op1, remaining) = self.eval(symbols.clone());
                    let (op2, remaining) = self.eval(remaining);
                    let mut ops: Vec<Symbol> = op1.to_vec();
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
                    let (op1, remaining) = self.eval(symbols.clone());
                    let ops: Vec<Symbol> = op1.to_vec();
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
                Symbol::Variable(key) => {
                    println!("variable key: {}", key);
                    let index = self.keys.get(&key).expect("Unknown variable!").clone();

                    if !self.lines[index].has_value {
                        let (op1, remaining) = self.eval(self.mem[index].clone());
                        op1.to_vec().extend(remaining);
                        self.mem[index] = op1;
                        self.lines[index].has_value = true;
                    }
                    let mut extended = self.mem[index].clone();
                    extended.extend(symbols);
                    symbols = extended;
                }
                _ => {
                    panic!("Please implement {:?}", s);
                }
            };
        }
        (value, symbols)
    }
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