use std::io;
use std::cmp;

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
    let n = parse_input!(input_line, usize);
    let mut xmin = i32::max_value();
    let mut xmax = i32::min_value();
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i64);
        xmin = cmp::min(x, xmin);
        xmax = cmp::max(x, xmax);
        ys.push(y);
    }
    ys.sort();
    let main_y = ys.get(ys.len() / 2).unwrap();
    let mut l = ys.iter().fold(0u64, |acc, y| {acc + (y - main_y).abs() as u64});
    l += (xmax as i64 - xmin as i64) as u64;

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", l);
}
