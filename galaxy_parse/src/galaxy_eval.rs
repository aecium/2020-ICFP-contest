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
    if let Ops::Ap(left, _) = op{
        if op.arity() > 0 {
            EvalOpsResult::Noop
        } else {
            //EvalOpsResult::NewOps(Ops::Nil)
            //find out my function by going down the left until it's not an Ap
            let opType = find_op_type(op);
            //look up the arity of that function (maybe not needed)
            //create an iterator over all A0 elements of the list (but not the root!)
            
            //recurse and eval_ops each of them (map?!) (literals will remain literals, while aps will resolve)
            //use iterator to compute A0 result
            //return

            EvalOpsResult::NewOps(Ops::Nil)
        }
    } else {
        EvalOpsResult::Noop
    }
}