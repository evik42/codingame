use std::io;

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
    let r = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, usize);
    let mut line = vec![r];
    for i in 1..l {
        let mut count = 0;
        let mut current = *(line.first().unwrap());
        let mut next_line = Vec::new();
        for c in line {
            if c == current {
                count += 1;
            } else {
                next_line.push(count);
                next_line.push(current);
                count = 1;
                current = c;
            }
        }
        next_line.push(count);
        next_line.push(current);
        line = next_line;
    }
    println!("{}", line.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
}
