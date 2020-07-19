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
            
            //EvalOpsResult::NewOps(Ops::Nil)
            //find out my function by going down the left until it's not an Ap
            let op_type = find_op_type(op);
            //look up the arity of that function (maybe not needed)
            //create an iterator over all A0 elements of the list (but not the root!)
            let mut arg_iter = left.into_iter().chain(right.into_iter());
            //use iterator to compute A0 result
            let replacement_op = eval_op(op_type, &mut arg_iter);
            //recurse and see if the root can be reduced again (those crafty combinators)
            //return
            EvalOpsResult::NewOps(Ops::Nil)
        }
        } else {
            EvalOpsResult::Noop
        }
}   

fn eval_op<'a> (op: &Ops, input : &mut std::iter::Chain<OpsIterator<'a>, OpsIterator<'a>>) -> Ops {
    match op {
        Cons => {
//            let first = input.next().unwrap();
//            let second = input.next().unwrap();
//            let second_simplified = eval_ops(second);
//            let first_num = match eval_ops(first) {
//                None
//            }
        },
        Car => {

        },
        Cdr => {

        },
        Nil => {

        },
        IsNil => {

        },
        Inc => {
            let inop = input.next().unwrap();
            let op = eval_ops(inop);
            match op {
                EvalOpsResult::Noop => {
                    // no new operation was found, use the old one and evaluate
                    if let Ops::Literal(x) = inop {
                        return Ops::Literal(x + 1);
                    } else {
                        panic!("Inc doesn't understand how to increment a {:?}", inop)
                    }
                },
                EvalOpsResult::NewOps(newop) => {
                    if let Ops::Literal(x) = newop {
                        return Ops::Literal(x + 1);
                    } else {
                        panic!("Inc doesn't understand how to increment a {:?}", newop);
                    }
                }
            };
        },
        Dec => {
          
        },
        Sum => {

        },
        Mul => {

        },
        Div => {

        },
        Neg => {

        },
        Eq => {

        },
        Lt => {

        },
        SComb => {

        },
        BComb => {

        },
        CComb => {

        },
        IComb => {

        },
        TChoice => {

        },
        FChoice => {

        },
        _ => {
            panic!("Not implemented")
        }
    }
    Ops::Nil
}