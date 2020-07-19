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
            let arg_iter = left.into_iter().chain(right.into_iter());
            //use iterator to compute A0 result
            let replacement_op = eval_op(op_type, arg_iter);
            //recurse and see if the root can be reduced again (those crafty combinators)
            //return
            EvalOpsResult::NewOps(Ops::Nil)
        }
        } else {
            EvalOpsResult::Noop
        }
}   

fn eval_op<'a> (op: &Ops, input : std::iter::Chain<OpsIterator, OpsIterator>) -> Ops {
       //TODO stub for Aecium
    match op {
        Cons => {

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
            let _inop = match input.next(){
                Some(x) => x,
                None => panic!("No opreand past to Inc.")
            };
            let _op = eval_ops(_inop);
            match _op {
                Noop => {
                    return _inop.0 + 1;
                },
                EvalOpsResult::NewOps(_newop) => {
                    return _newop + 1;
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