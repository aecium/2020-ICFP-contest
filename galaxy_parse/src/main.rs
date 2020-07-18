mod galaxy_parse;
use galaxy_parse::*;

fn main() {
    dbg!(galaxy_parse::parse_line("42"));
}
