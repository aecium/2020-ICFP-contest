use nom::IResult;

#[derive(Debug, Clone)]
pub enum Ops {
    Literal(i128),
    Variable(String),
    Ap(Box<Ops>, Box<Ops>),
    List(Vec<Ops>),
    Cons,
    Car,
    Cdr,
    Nil,
    IsNil,
    Inc,
    Dec,
    Sum,
    Mul,
    Div,
    Neg,
    Eq,
    Lt,
    SComb,
    BComb,
    CComb,
    IComb,
    TChoice,
    FChoice,
}

pub struct OpsIterator<'a> {
    ops: Vec<&'a Ops>
}

impl<'a> Iterator for OpsIterator<'a> {
    type Item = &'a Ops;

    fn next(&mut self) -> Option<&'a Ops> {
        self.ops.pop()
    }
}

impl Ops {
    pub fn arity (&self) -> i128 {
        match self {
            Ops::Literal(_) | Ops::Variable(_) | Ops::List(_) | Ops::Nil => 0,
            Ops::Car | Ops::Cdr | Ops::Inc | Ops::Dec | Ops::Neg | Ops::IComb => 1,
            Ops::Cons | Ops::Sum | Ops::Mul | Ops::Div | Ops::TChoice | Ops::FChoice => 2,
            Ops::IsNil | Ops::SComb | Ops::BComb | Ops::CComb => 3,
            Ops::Eq | Ops::Lt => 4,
            Ops::Ap(left, _) => left.arity() - 1
        }
    }

    
}
impl<'a> IntoIterator for &'a Ops {
    type Item = &'a Ops;
    type IntoIter = OpsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut nodes = Vec::<&'a Ops>::new();
        //push all the right kids along the left side
        let mut current_node : &'a Ops = self;
        while let Ops::Ap(left, right ) = current_node {
            nodes.push(right);
            current_node = left;
        }
        return OpsIterator {
            ops: nodes
        }
    }
}

fn parse_literal(input : &'_ str) -> IResult<&'_ str, Ops> {
    
    nom::combinator::map_parser(
        nom::bytes::complete::take_till(|c| c == ' '), |digits: &str| -> IResult<&'_ str, Ops>{
        match digits.parse::<i128>() {
            Ok(num) => Ok((&input[digits.len()..input.len()], Ops::Literal(num))),
            _ => IResult::Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::IsNot)))
        }
    })(input)
}

fn parse_func(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::branch::alt((
        //IsNil
        nom::combinator::map(nom::bytes::complete::tag("isnil"), |_| Ops::IsNil),
        //ap
        nom::combinator::map(nom::sequence::tuple((
            nom::bytes::complete::tag("ap"),
            nom::sequence::preceded(nom::character::complete::space1, parse_symbol),
            nom::sequence::preceded(nom::character::complete::space1, parse_symbol),
        )), |(_, left, right)| Ops::Ap(Box::new(left), Box::new(right))),
        //cons
        nom::combinator::map(nom::bytes::complete::tag("cons"), |_| Ops::Cons),
        //nil
        nom::combinator::map(nom::bytes::complete::tag("nil"), |_| Ops::Nil),
        //inc
        nom::combinator::map(nom::bytes::complete::tag("inc"), |_| Ops::Inc),
        //dec
        nom::combinator::map(nom::bytes::complete::tag("dec"), |_| Ops::Dec),
        //sum
        nom::combinator::map(nom::bytes::complete::tag("add"), |_| Ops::Sum),
        //mul
        nom::combinator::map(nom::bytes::complete::tag("mul"), |_| Ops::Mul),
        //div
        nom::combinator::map(nom::bytes::complete::tag("div"), |_| Ops::Div),
        //neg
        nom::combinator::map(nom::bytes::complete::tag("neg"), |_| Ops::Neg),
        //Car
        nom::combinator::map(nom::bytes::complete::tag("car"), |_| Ops::Car),
        //Cdr
        nom::combinator::map(nom::bytes::complete::tag("cdr"), |_| Ops::Cdr),
        //eq
        nom::combinator::map(nom::bytes::complete::tag("eq"), |_| Ops::Eq),
        //lt
        nom::combinator::map(nom::bytes::complete::tag("lt"), |_| Ops::Lt),
        //SComb
        nom::combinator::map(nom::bytes::complete::tag("s"), |_| Ops::SComb),
        //CComb
        nom::combinator::map(nom::bytes::complete::tag("c"), |_| Ops::CComb),
        //BComb
        nom::combinator::map(nom::bytes::complete::tag("b"), |_| Ops::BComb),
        //TChoice
        nom::combinator::map(nom::bytes::complete::tag("t"), |_| Ops::TChoice),
        //FChoice
        nom::combinator::map(nom::bytes::complete::tag("f"), |_| Ops::FChoice),
        //IComb
        nom::combinator::map(nom::bytes::complete::tag("i"), |_| Ops::IComb),
    ))(input)
}

fn parse_variable(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::combinator::map(
        nom::sequence::preceded(nom::character::complete::char(':'),nom::character::complete::alphanumeric1),
        |label : &str| Ops::Variable(label.to_owned())
    )(input)
}

fn parse_list(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::bytes::complete::tag("( "),
            nom::multi::separated_list(
                nom::bytes::complete::tag(" , "),
                parse_symbol
            ),
            nom::branch::alt((
                nom::bytes::complete::tag(")"),
                nom::bytes::complete::tag(" )")
            ))
        )),
        | (_open_paren, mut symbols, _close_paren) | {
            symbols.reverse();
            Ops::List(symbols)}
    ) (input)
}

fn parse_symbol(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::branch::alt((
        parse_list,
        parse_literal,
        parse_variable,
        parse_func
    ))(input)
}

pub fn parse_line(input : &'_ str) -> IResult<&'_ str, (&'_ str, Ops)> {
    nom::combinator::all_consuming(
        nom::combinator::map(nom::sequence::tuple((
            nom::bytes::complete::take_till(|c| c == ' '),
            //nom::sequence::preceded(nom::character::complete::char(':'), nom::character::complete::alphanumeric1),
            nom::sequence::preceded(nom::character::complete::space1,nom::character::complete::char('=')),
            nom::sequence::preceded(nom::character::complete::space1,parse_symbol)
        )), |(label, _, op)| (label, op))
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_literal() {
        let (remainder, op) = (parse_literal("42")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Literal(42) => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_literal_with_more() {
        let (remainder, op) = (parse_literal("42 27")).unwrap();
        assert_eq!(remainder, " 27");
        match op {
            Ops::Literal(42) => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_func_nil() {
        let (remainder, op) = (parse_func("nil")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Nil => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_func_cons() {
        let (remainder, op) = (parse_func("cons")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Cons => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_sym_1() {
        let (remainder, op) = (parse_symbol("1")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Literal(1) => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_sym_cons() {
        let (remainder, op) = (parse_symbol("cons")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Cons => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_sym_ap_1_2() {
        let (remainder, op) = (parse_symbol("ap 1 2")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Ap(left, right) => {
                if let left = Ops::Literal(1) {
                    // correct
                } else {
                    assert!(false);
                };
                if let right = Ops::Literal(2) {
                    // correct
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_list_empty() {
        let (remainder, op) = (parse_symbol("( )")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::List(x) => assert_eq!(x.len(), 0),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_list_single() {
        let (remainder, op) = (parse_symbol("( 1 )")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::List(x) => assert_eq!(x.len(), 1),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_list_single_op() {
        let (remainder, op) = (parse_symbol("( nil )")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::List(x) => assert_eq!(x.len(), 1),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_sym_isnil() {
        let (remainder, op) = (parse_symbol("isnil")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::IsNil => assert!(true),
            _ => assert!(false)
        };
    }
    #[test]
    fn test_parse_list_multi() {
        let (remainder, op) = (parse_symbol("( 1 , 2 , 3 )")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::List(x) => assert_eq!(x.len(), 3),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_list_multi_ops() {
        let (remainder, op) = (parse_symbol("( ap inc 1 , 2 , 3 )")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::List(x) => assert_eq!(x.len(), 3),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_variable() {
        let (remainder, op) = (parse_symbol(":1162")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Variable(label) => assert_eq!(label, "1162"),
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_variable_under_ap() {
        let (remainder, op) = (parse_symbol("ap :1162 1")).unwrap();
        assert_eq!(remainder, "");
        match op {
            Ops::Ap(left, right) => {
                match *left {
                    Ops::Variable(label) => assert_eq!(label, "1162"),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
    }
    #[test]
    fn test_parse_line_1116() {
        let (remainder, op) = (parse_symbol("ap ap s ap ap b c isnil car")).unwrap();
        assert_eq!(remainder, "");
    }
    #[test]
    fn test_iter_simple() {
        let optree = Ops::Ap(Box::new(Ops::Sum), Box::new(Ops::Literal(1)));
        let mut iter = optree.into_iter();
        match iter.next() {
            Some(Ops::Literal(one)) => assert_eq!(*one, 1i128),
            _ => assert!(false)
        };
        match iter.next() {
            None => assert!(true),
            _ => assert!(false)
        };

    }
    #[test]
    fn test_iter_complex() {
        let optree = Ops::Ap(Box::new(Ops::Ap(Box::new(Ops::IsNil),Box::new(Ops::Nil))), Box::new(Ops::Ap(Box::new(Ops::Inc),Box::new(Ops::Literal(1)))));
        let mut iter = optree.into_iter();
        match iter.next() {
            Some(Ops::Nil) => {},
            _ => assert!(false)
        };
        match iter.next() {
            Some(Ops::Ap(left,right)) => {
                match **left {
                    Ops::Inc => {}
                    _ => assert!(false)
                }
                match **right {
                    Ops::Literal(one) => assert_eq!(one, 1i128),
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        };

    }
}