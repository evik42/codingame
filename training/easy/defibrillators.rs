use std::io;
use std::f64;

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

macro_rules! string_to_radians {
    ($s:expr) => (parse_input!($s.replace(",", "."), f64).to_radians())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = string_to_radians!(input_line.trim().to_string());
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = string_to_radians!(input_line.trim().to_string());
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut closest = "".to_string();
    let mut distance = f64::MAX;
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib_string = input_line.trim_right().to_string();
        let mut defib = defib_string.split(';');
        let (id, name, addr, phone, dlon, dlat) = (defib.next(), defib.next(), defib.next(), defib.next(), defib.next(), defib.next());
        let defibd = calculate_distance(lon, lat, string_to_radians!(dlon.unwrap()), string_to_radians!(dlat.unwrap()));
        if defibd < distance {
            distance = defibd;
            closest = name.unwrap().to_owned();
        }
    }

    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("{}", closest);
}

fn calculate_distance(ulon: f64, ulat: f64, dlon: f64, dlat: f64) -> f64 {
    let x = (dlon - ulon) * ((ulat + dlat) / 2.0).cos();
    let y = dlat - ulat;
    x.powi(2) + y.powi(2)
}
