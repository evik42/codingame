use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*);
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
    let l = parse_input!(input_line, usize);
    input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);
    input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let t : Vec<_> = input_line.trim().chars().collect();
    let mut res = String::with_capacity(t.len() * l * h + h);

    for i in 0..h as usize {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.to_string();

        for c in t.iter().cloned() {
            let offset = match c.to_digit(36) {
                Some(o) => (o as usize) - 10,
                None    => 26,
            };
            let startIndex = offset * l;
            let stopIndex = startIndex + l;
            res.push_str(&row[startIndex..stopIndex]);
        }
        res.push('\n');
    }

    println!("{}", res);
}
