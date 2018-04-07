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
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut mov = String::new();

        ty = if ty < light_y {
            mov.push_str("S");
            ty + 1
        } else if ty > light_y {
            mov.push_str("N");
            ty - 1
        } else {
            ty
        };
        tx = if tx < light_x {
            mov.push_str("E");
            tx + 1
        } else if tx > light_x {
            mov.push_str("W");
            tx - 1
        } else {
            tx
        };


        println!("{}", mov); // A single line providing the move to be made: N NE E SE S SW W or NW
    }
}
