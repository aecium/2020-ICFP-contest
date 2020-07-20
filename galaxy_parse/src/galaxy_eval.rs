use crate::galaxy_parse::*;
use crate::Galaxy;

pub fn eval (label: &str, galaxy: &mut Galaxy) {
    let root = galaxy.data.get(label);
    dbg!(&root);
    let result = match root {
        Some(op) => eval_ops(op),
        None => panic!("That address isn't in this universe!")
    };
    if let EvalOpsResult::NewOps(op) = result {
        galaxy.data.insert(label.to_string(), op);
    }
    
}
enum EvalOpsResult {
    Noop,
    NewOps(Ops)
}

fn find_op_type(op : &Ops) -> &Ops{
    if let Ops::Ap(left,_) = op {
        find_op_type(left)
    } else {
        op
    }
}

fn eval_ops (op : &Ops) -> EvalOpsResult {
    if let Ops::Ap(left, right) = op{
        if op.arity() > 0 {
            EvalOpsResult::Noop
        } else {
            //find out my function by going down the left until it's not an Ap
            let op_type = find_op_type(op);
            //look up the arity of that function (maybe not needed)
            //create an iterator over all A0 elements of the list (but not the root!)
            let mut arg_iter = op.into_iter();
            //use iterator to compute A0 result
            let replacement_op = eval_op(op_type, &mut arg_iter);
            //recurse and see if the root can be reduced again (those crafty combinators)
            //return
            match eval_ops(&replacement_op) {
                EvalOpsResult::Noop => EvalOpsResult::NewOps(replacement_op),
                EvalOpsResult::NewOps(new) => EvalOpsResult::NewOps(new)
            }
            
        }
        } else {
            EvalOpsResult::Noop
        }
}   

fn eval_op<'a> (op: &Ops, input : &mut OpsIterator) -> Ops {
    dbg!(op);
    match op {
        Ops::Cons => {
            let first = input.next().unwrap();
            let second = input.next().unwrap();
            dbg!(first);
            dbg!(second);
            let terms = match (eval_ops(first), eval_ops(second)) {
                (EvalOpsResult::Noop, EvalOpsResult::Noop) => (first.clone(),second.clone()),
                (EvalOpsResult::Noop, EvalOpsResult::NewOps(right)) => (first.clone(),right),
                (EvalOpsResult::NewOps(left), EvalOpsResult::Noop) => (left,second.clone()),
                (EvalOpsResult::NewOps(left), EvalOpsResult::NewOps(right)) => (left,right),
            };
            match terms {
                (Ops::Nil, Ops::Nil) => Ops::List(Vec::new()),
                (first, Ops::Nil) => Ops::List(vec![first]),
                (head,mut tail) => {
                    if let Ops::List(ref mut vec) = tail {
                        vec.push(head);
                        tail
                    } else {
                        Ops::List(vec![head, tail])
                    }
                }
            }
        },
        Ops::Car => {
            unimplemented!()
        },
        Ops::Cdr => {
            unimplemented!()
        },
        Ops::IsNil => {
            unimplemented!()
        },
        Ops::Inc => {
            let inop = input.next().unwrap();
            let op = eval_ops(inop);
            match op {
                EvalOpsResult::Noop => {
                    // no new operation was found, use the old one and evaluate
                    if let Ops::Literal(x) = inop {
                        Ops::Literal(x + 1)
                    } else {
                        panic!("Inc doesn't understand how to increment a {:?}", inop)
                    }
                },
                EvalOpsResult::NewOps(newop) => {
                    if let Ops::Literal(x) = newop {
                        Ops::Literal(x + 1)
                    } else {
                        panic!("Inc doesn't understand how to increment a {:?}", newop);
                    }
                }
            }
        },
        Ops::Dec => {
            unimplemented!()
        },
        Ops::Sum => {
            unimplemented!()
        },
        Ops::Mul => {
            unimplemented!()
        },
        Ops::Div => {
            unimplemented!()
        },
        Ops::Neg => {
            unimplemented!()
        },
        Ops::Eq => {
            unimplemented!()
        },
        Ops::Lt => {
            unimplemented!()
        },
        Ops::SComb => {
            unimplemented!()
        },
        Ops::BComb => {
            unimplemented!()
        },
        Ops::CComb => {
            unimplemented!()
        },
        Ops::IComb => {
            unimplemented!()
        },
        Ops::TChoice => {
            unimplemented!()
        },
        Ops::FChoice => {
            unimplemented!()
        },
        _ => {
            panic!("Not implemented")
        }
    }
}