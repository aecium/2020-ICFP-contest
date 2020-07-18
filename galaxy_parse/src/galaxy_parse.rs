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

impl Ops {
    fn named_parse(input: &str) -> Self {
        match input {
            _ => Self::Nil
        }
    }
}

fn take_token(input : &'_ str) -> IResult<&'_ str, &'_ str> {
    nom::bytes::complete::is_not(" ")(input)
}

fn parse_literal(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::combinator::map_parser(nom::character::complete::digit1, |digits: &str| -> IResult<&'_ str, Ops>{
        match input.parse::<i128>() {
            Ok(num) => Ok((&input[input.len()..input.len()], Ops::Literal(num))),
            _ => IResult::Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::IsNot)))
        }
    })(input)
}

fn parse_func(input : &'_ str) -> IResult<&'_ str, Ops> {
    nom::combinator::map_parser(nom::character::complete::alpha1, |opName: &str| -> IResult<&'_ str, Ops>{
        match opName {
            "nil" => Ok((&input[input.len()..input.len()], Ops::Nil)),
            "cons" => Ok((&input[input.len()..input.len()], Ops::Cons)),
            "ap" => Ok((&input[input.len()..input.len()], Ops::Nil)), //TODO this is definitely wrong, time for recursion

            _ => IResult::Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::IsNot)))
        }
    })(input)
}

pub fn parse_line(input : &'_ str) -> IResult<&'_ str, Ops> {
    Ok((input, Ops::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_take_token() {
        assert_eq!(take_token("42 "),Ok((" ", "42")))
    }
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
}