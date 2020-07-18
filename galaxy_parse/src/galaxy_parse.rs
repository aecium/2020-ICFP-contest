use nom::IResult;

#[derive(Debug)]
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
    Eq,
    Lt,
    SComb,
    BComb,
    CComb,
    IComb,
    TChoice,
    FChoice,
}

impl Ops {
    fn arity(&self) -> usize {
        0
    }
}

fn parse_literal(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::combinator::map_parser(nom::character::complete::digit1, |digits: &str| -> IResult<&'_ str, Ops>{
        match digits.parse::<i128>() {
            Ok(num) => Ok((&input[digits.len()..input.len()], Ops::Literal(num))),
            _ => IResult::Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::IsNot)))
        }
    })(input)
}

fn parse_func(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::branch::alt((
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
        //Car
        nom::combinator::map(nom::bytes::complete::tag("car"), |_| Ops::Car),
        //Cdr
        nom::combinator::map(nom::bytes::complete::tag("cdr"), |_| Ops::Cdr),
        //IsNil
        nom::combinator::map(nom::bytes::complete::tag("isnil"), |_| Ops::IsNil),
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
        | (_open_paren, symbols, _close_paren) | Ops::List(symbols)
    ) (input)
}



pub fn parse_symbol(input : &'_ str) -> IResult<&'_ str, Ops> {
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
            nom::sequence::preceded(nom::character::complete::char(':'), nom::character::complete::alphanumeric1),
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
}