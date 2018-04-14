use std::io;
use std::collections::BTreeSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut mem = BTreeSet::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut telephone = input_line.trim().to_string();
        while ! telephone.is_empty() {
            mem.insert(telephone.clone());
            telephone.pop();
        }
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    // The number of elements (referencing a number) stored in the structure.
    println!("{}", mem.len());
}
