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

macro_rules! read_single_input {
    ($t:ident) => (
        {
            let mut il = String::new();
            io::stdin().read_line(&mut il).unwrap();
            parse_input!(il, $t)
        }
    )
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let n = read_single_input!(i32); // the number of temperatures to analyse
    let mut temps = String::new();
    io::stdin().read_line(&mut temps).unwrap();
    let temps = temps.trim().to_string(); // the n temperatures expressed as integers ranging from -273 to 5526

    let minmax = |acc:i32, t:i32| match (acc, t) {
         (acc, t) if t.abs() < acc.abs() => t,
         (acc, t) if t.abs() > acc.abs() => acc,
         (acc, t) if t > acc             => t,
         (acc, _)                        => acc,
    };

    let result = temps.split_whitespace().map(|s| {s.parse::<i32>().unwrap()}).fold(5527, minmax);

    println!("{}", if result < 5527 {result} else {0});
}
