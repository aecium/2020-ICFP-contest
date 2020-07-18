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
    Cons(Box<Ops>, Box<Ops>),
    Nil,
}

impl Ops {
    fn named_parse(input: &str) -> Self {
        match input {
            _ => Self::Nil
        }
    }
    fn parse_literal(input : i128) -> Self {
        Self::Nil
    }
}

fn tokenize(input : &'_ str) -> IResult<&'_ str, &'_ str> {
    nom::bytes::complete::is_not(" ")(input)
}

fn parse_token(input : &'_ str) -> IResult<&'_ str, Ops> {
    Ok((input, Ops::Nil))
}

pub fn parse_line(input : &'_ str) -> IResult<&'_ str, Ops> {
    Ok((input, Ops::Nil))
}