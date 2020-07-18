use nom::IResult;

#[derive(Debug)]
pub enum Ops {
    Literal(i128),
    Bind(usize, Box<Ops>), // Assignment Equality / Bind(memloc, ops)
    /*
    Inc,
    Dec,
    Sum,
    Mul,
    Div,
    Eq,
    */
    
    Ap(Box<Ops>, Box<Ops>),
    Cons,
    Nil,
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
        nom::combinator::map(nom::bytes::complete::tag("nil"), |_| Ops::Nil)
    ))(input)
}

pub fn parse_symbol(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::branch::alt((
        parse_literal,
        parse_func
    ))(input)
}

pub fn _parse_line(input : &'_ str) -> IResult<&'_ str, Ops> {
    //TODO handle the beginning of the line
    parse_symbol(input)
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
    fn test_parse_sym_ap_ap_cons_7_nil() {
        let (remainder, op) = (parse_symbol("ap ap cons 7 nil")).unwrap();
        assert_eq!(remainder, "");
        dbg!(op);
    }
}