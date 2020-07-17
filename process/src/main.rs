use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum Symbol {
    Equality,
    Successor,
    Predecessor,
    Sum,
    Product,
    IntegerDivision,
    StrictLessThan,
    Modulate,
    Demodulate,
    SendNow, //Send
    Negate,
    FuncApp, //Function Application
    SC, //S Combinator
    CC, //C Combinator
    BC, //B Combinator
    TC, // True (K Combinator)
    FC, // False
    PowerOfTwo,
    IC, //i Combinator
    Cons, //(or Pair)
    Car, //(First)
    Cdr, //(Tail)
    Nil, //(Empty List)
    IsNil, //(Is Empty List)
    LCS, //List Construction Syntax
    Vector,
    Draw,
    Checkerboard,
    MulDraw, //Multiple Draw
    ModList, //Modulate List
    Send0,
    Is0,
    Interact,
    InterProt, //Interaction Protocol
    StatesessDraw, //Stateless Drawing Protocol
    StatefullDraw, //Statefull Drawing Protocol
    Galaxy,
}

fn equality(x: i32, y: i32) -> bool {
    true
}

fn successor(x: i32) -> i32{
    0
}

fn predecessor(x: i32) -> i32 {
    0
}

fn sum(x: i32, y: i32) -> i32 {
    0
}

fn product(x: i32, y: i32) -> i32  {
    0
}

fn integer_division(x: i32, y: i32) -> i32  {
    0
}

fn strictless_than(x: i32, y: i32) -> bool  {
    true
}

fn modulate(x: i32) {

}

fn demodulate(s: String) -> i32 {
    0
}

fn send_now(s: &str) {

}

fn negate(x: i32) -> i32 {
    0
}

fn func_app() { //Function Application

}

fn sc() { //S Combinator

}

fn cc(){ //C Combinator

}

fn bc() { //B Combinator

}

fn tcom() { // True (K Combinator)

}

fn fcom() { // False

}

fn power_of_two() {

}

fn ic() { //i Combinator

}

fn cons() { //(or Pair)

}

fn car() { //(First)

}

fn cdr() { //(Tail)

}

fn nil() { //(Empty List)

}

fn is_nil() { //(Is Empty List)

}

fn lcs() { //List Construction Syntax

}

fn vector() {

}

fn draw() {

}

fn checkerboard() {

}

fn mul_draw() { //Multiple Draw

}

fn mod_list() { //Modulate List

}

fn send0() {

}

fn is0() {

}

fn interact() {

}

fn inter_prot() { //Interaction Protocol

}

fn statesess_draw() { //Stateless Drawing Protocol

}

fn statefull_draw() { //Statefull Drawing Protocol

}

fn galaxy() {

}

fn main() {
    // File hosts must exist in current path before this produces output
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(out) = line {
                println!("{}", out);
            }
        }
    }
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}