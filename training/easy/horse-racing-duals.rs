use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

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
    let mut strengths = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        strengths.push(pi);
    }
    strengths.sort();
    let (diff, _) = strengths.iter().fold((10000001, -30000000), minimum_diff);

    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("{}", diff);
}

fn minimum_diff(acc: (i32, i32), item: &i32) -> (i32, i32) {
    let (d, prev) = acc;
    let d = if item - prev < d {item - prev} else {d};
    (d, item.to_owned())
}
