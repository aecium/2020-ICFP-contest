use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Var {
    src: String,
    value: Vec<i64>,
}

fn main() {
    let mut vars: HashMap<String, Var> = HashMap::new();

    if let Ok(lines) = read_lines("galaxy.txt") {
        let mut i = 0;
        for line in lines {
            i+=1;
            if let Ok(l) = line {
                let mut ap = 0;
                println!("{}", l);
                let mut sym_iter = l.split_whitespace();
                let key = sym_iter.next().expect("Oops, no variable name.");
                let _eq = sym_iter.next().expect("Oops, no equals.");
                let mut symbols: Vec<Symbol> = Vec::new();

                print!("{} = ", key);
                for s in sym_iter {
                    let num = s.parse::<i64>();
                    if s.eq("ap") {
                        ap += 1;
                        continue;
                    } else if s.eq("cons") {
                        symbols.push(Symbol::Cons);
                    } else if s.eq("nil") {
                        symbols.push(Symbol::Value(Vec::new()));
                    } else if num.is_ok() {
                        let mut v = Vec::new();
                        v.push(num.unwrap());
                        symbols.push(Symbol::Value(v));
                    } else {
                        println!();
                        println!("TODO: {}", s);
                    }
                    print!("{},", s);
                }
                println!();

                let (value, _remaining) = eval(&vars, &symbols);
                println!("key: {} - val: {:?} - ap: {}", key, value, ap);

                vars.insert(key.to_string(), Var{src: l, value: value});
            }
            if i > 13 {
                break;
            }
        }

        println!();
        println!("Done!  Values:");
        for var in vars.values() {
            println!("src: {}\n val: {:?}\n", var.src, var.value);
        }
    }
}

fn eval(vars: &HashMap<String, Var>, symbols: &[Symbol]) -> (Vec<i64>, Vec<Symbol>) {
    let mut value: Vec<i64> = Vec::new();
    let mut symbols = symbols.to_vec();
    while symbols.len() > 0 {
        let s = symbols.remove(0);
        match s {
            Symbol::Value(val) => {
                value = val.to_vec();
                break;
            },
            Symbol::Cons => {
                let (op1, remaining) = eval(&vars, &symbols);
                let (op2, remaining) = eval(&vars, &remaining);
                let mut values: Vec<i64> = Vec::new();
                values.extend(op1);
                values.extend(op2);
                symbols = remaining.to_vec();
                symbols.push(Symbol::Value(values));

            },
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
    Cons, //(or Pair)
}