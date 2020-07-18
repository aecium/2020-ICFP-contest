mod galaxy_parse;
use galaxy_parse::*;

fn main() {
    dbg!(parse_line(":1102 = ap ap cons :1097 ap ap cons :1098 ap ap cons :1099 ap ap cons :1100 nil"));
}
