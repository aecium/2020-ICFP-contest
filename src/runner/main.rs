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

    println!("Evaluating entrypoint {} ({})", entrypoint, state.get_line(entrypoint).src);

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
                    symbols.push(Symbol::Ap);
                } else if s.starts_with(":") {
                    symbols.push(Symbol::Variable(s.to_string()));
                } else if s.eq("cons") {
                    symbols.push(Symbol::Cons);
                } else if s.eq("neg") {
                    symbols.push(Symbol::Neg);
                } else if s.eq("nil") {
                    symbols.push(Symbol::Value(Vec::new()));
                } else if s.eq("isnil") {
                    symbols.push(Symbol::IsNil);
                } else if s.eq("inc") {
                    symbols.push(Symbol::Inc);
                } else if s.eq("dec") {
                    symbols.push(Symbol::Dec);
                } else if s.eq("add") {
                    symbols.push(Symbol::Sum);
                } else if s.eq("mul") {
                    symbols.push(Symbol::Mul);
                } else if s.eq("div") {
                    symbols.push(Symbol::Div);
                } else if s.eq("car") {
                    symbols.push(Symbol::Car);
                } else if s.eq("cdr") {
                    symbols.push(Symbol::Cdr);
                } else if s.eq("eq") {
                    symbols.push(Symbol::Eq);
                } else if s.eq("lt") {
                    symbols.push(Symbol::Lt);
                } else if s.eq("s") {
                    symbols.push(Symbol::SComb);
                } else if s.eq("c") {
                    symbols.push(Symbol::CComb);
                } else if s.eq("b") {
                    symbols.push(Symbol::BComb);
                } else if s.eq("t") {
                    symbols.push(Symbol::TChoice);
                } else if s.eq("f") {
                    symbols.push(Symbol::FChoice);
                } else if s.eq("i") {
                    symbols.push(Symbol::IComb);
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
        println!("eval({:?})", symbols);
        let mut value: Vec<Symbol> = Vec::new();
        let mut remaining_symbols = symbols.to_vec();
        let mut ap = 0;
        while remaining_symbols.len() > 0 {
            println!("while remaining: {:?}", remaining_symbols);
            let s = remaining_symbols.remove(0);
            match s.clone() {
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
                    extended.extend(remaining_symbols);
                    remaining_symbols = extended;
                }
                Symbol::Ap => {
                    ap += 1;
                },
                Symbol::Value(_val) => {
                    value.push(s.clone());
                    break;
                },
                Symbol::Nil => {
                    value.push(Symbol::Value(Vec::new()));
                    break;
                },
                Symbol::Inc => {
                    if ap < 1 {
                        value.push(Symbol::Inc);
                    } else {
                        ap -= 1;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        remaining_symbols = remaining;
                        let val = unwrap_single_value(op1);
                        value.push(wrap_single_val(val+1));
                    }
                    break;
                },
                Symbol::Dec => {
                    if ap < 1 {
                        value.push(Symbol::Inc);
                    } else {
                        ap -= 1;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        remaining_symbols = remaining;
                        let val = unwrap_single_value(op1);
                        value.push(wrap_single_val(val-1));
                    }
                    break;
                },
                Symbol::Sum => {
                    if ap < 2 {
                        value.push(Symbol::Sum);
                        if ap == 1 {
                            ap-=1;
                            value.push(Symbol::Ap);
                            panic!("Extra ap detected.");
                        }
                    } else {
                        ap -= 2;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        let (op2, remaining) = self.eval(remaining);
                        remaining_symbols = remaining;
                        let val1 = unwrap_single_value(op1);
                        let val2 = unwrap_single_value(op2);
                        value.push(wrap_single_val(val1+val2));
                    }
                    break;
                },
                Symbol::Mul => {
                    if ap < 2 {
                        value.push(Symbol::Sum);
                        if ap == 1 {
                            ap-=1;
                            value.push(Symbol::Ap);
                            panic!("Extra ap detected.");
                        }
                    } else {
                        ap -= 2;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        let (op2, remaining) = self.eval(remaining);
                        remaining_symbols = remaining;
                        let val1 = unwrap_single_value(op1);
                        let val2 = unwrap_single_value(op2);
                        value.push(wrap_single_val(val1*val2));
                    }
                    break;
                },
                Symbol::Div => {
                    if ap < 2 {
                        value.push(Symbol::Sum);
                        if ap == 1 {
                            ap-=1;
                            value.push(Symbol::Ap);
                            panic!("Extra ap detected.");
                        }
                    } else {
                        ap -= 2;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        let (op2, remaining) = self.eval(remaining);
                        remaining_symbols = remaining;
                        let val1 = unwrap_single_value(op1);
                        let val2 = unwrap_single_value(op2);
                        value.push(wrap_single_val(val1/val2));
                    }
                    break;
                },
                Symbol::Eq => {
                    if ap < 2 {
                        value.push(Symbol::Sum);
                        if ap == 1 {
                            ap-=1;
                            value.push(Symbol::Ap);
                            panic!("Extra ap detected.");
                        }
                    } else {
                        ap -= 2;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        let (op2, remaining) = self.eval(remaining);
                        remaining_symbols = remaining;
                        let val1 = unwrap_single_value(op1);
                        let val2 = unwrap_single_value(op2);
                        value.push(if val1 == val2 {
                            Symbol::TChoice
                        } else {
                            Symbol::FChoice
                        })
                    }
                    break;
                },
                Symbol::Neg => {
                    if ap < 1 {
                        value.push(Symbol::Neg);
                    } else {
                        ap -= 1;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        remaining_symbols = remaining;
                        let val = unwrap_single_value(op1);
                        value.push(wrap_single_val(-val));
                    }
                    break;
                },
                Symbol::Cons => {
                    println!("Cons ap: {} - remaining {:?}", ap, remaining_symbols);
                    if ap < 2 {
                        if ap == 1 {
                            ap-=1;
                            value.push(Symbol::Ap);
                            panic!("Extra ap detected.");
                        }
                        value.push(Symbol::Cons);
                    } else {
                        ap -= 2;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
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
                                    println!("Symbol: {:?}", op);
                                    panic!("Awe, snap!");
                                }
                            }
                            
                        }
                        remaining_symbols = remaining.to_vec();
                        value.push(Symbol::Value(values));
                    }
                    break;
                },
                Symbol::SComb => {
                    if ap < 3 {
                        if ap > 0 {
                            panic!("Extra ap detected.");
                        }
                        value.push(Symbol::SComb);
                    } else {
                        ap -= 3;
                        let (x0, remaining) = self.eval(remaining_symbols.clone());
                        let (x1, remaining) = self.eval(remaining);
                        let (x2, remaining) = self.eval(remaining);
                        println!("SComb x0: {:?} x1: {:?} x2: {:?} remaining: {:?}", x0, x1, x2, remaining);
                        let mut replacement = Vec::new();
                        replacement.push(Symbol::Ap);
                        replacement.push(Symbol::Ap);
                        replacement.extend(x0);
                        replacement.extend(x2.clone());
                        replacement.push(Symbol::Ap);
                        replacement.extend(x1);
                        replacement.extend(x2);
                        replacement.extend(remaining);
                        remaining_symbols = replacement;
                    }
                    println!("SComb remaining: {:?}", &remaining_symbols);
                },
                Symbol::CComb => {
                    if ap < 3 {
                        if ap > 0 {
                            panic!("Extra ap detected.");
                        }
                        value.push(Symbol::CComb);
                    } else {
                        ap -= 3;
                        let (op1, remaining) = self.eval(remaining_symbols.clone());
                        let (op2, remaining) = self.eval(remaining);
                        let (op3, remaining) = self.eval(remaining);
                        println!("CComb op1: {:?} op2: {:?} op3: {:?} remaining: {:?}", op1, op2, op3, remaining);
                        let mut replacement = Vec::new();
                        replacement.push(Symbol::Ap);
                        replacement.push(Symbol::Ap);
                        replacement.extend(op1);
                        replacement.extend(op3);
                        replacement.extend(op2);
                        replacement.extend(remaining);
                        remaining_symbols = replacement;
                    }
                    println!("CComb remaining: {:?}", &remaining_symbols);
                },
                _ => {
                    println!("Please implement {:?}", s);
                    panic!("Please implement {:?}", s);
                }
            };
        }
        if ap > 0 {
            println!("ap > 0 (ap=={})", ap);
        }
        while ap > 0{
            value.insert(0, Symbol::Ap);
            ap -= 1;
            panic!("Extra ap detected.");
        }
        //println!("eval({:?}) -> (value: {:?}, remaining: {:?})", symbols, value, remaining_symbols);
        (value, remaining_symbols)
    }
}

fn unwrap_single_value(value: Vec<Symbol>) -> i64 {
    match value.len() {
        0 => panic!("Unpossible"),
        1 => {
            let op = value.get(0).expect("Unpossible");
            match op {
                Symbol::Value(vec) => {
                    match vec.len() {
                        1 => {
                            return vec.get(0).expect("Unpossible").clone();
                        },
                        _ => panic!("Unpossible"),
                    }
                },
                _ => panic!("Unpossible"),
            }
        },
        _ => panic!("Multiple Values!"),
    }
}

fn wrap_single_val(val: i64) -> Symbol {
    let mut vals = Vec::new();
    vals.push(val);
    Symbol::Value(vals)
}

#[derive(Clone,Debug)]
enum Symbol {
    Value(Vec<i64>),
    Variable(String),
    Ap,
    Cons, //(or Pair)
    Eq,

    Car, //(First)
    Cdr, //(Tail)
    Nil, //(Empty List)
    IsNil, //(Is Empty List)
    Inc,
    Dec,
    Sum,
    Mul,
    Div,
    Neg,
    Lt,
    SComb,
    BComb,
    CComb,
    IComb,
    TChoice,
    FChoice,

    /*
    Successor,
    Predecessor,
    Sum,
    Product,
    IntegerDivision,
    StrictLessThan,
    Modulate,
    Demodulate,
    SendNow, //Send
    Neg, //Negate
    FuncApp, //Function Application
    SC, //S Combinator
    CC, //C Combinator
    BC, //B Combinator
    TC, // True (K Combinator)
    FC, // False
    PowerOfTwo,
    IC, //i Combinator
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
    */
}