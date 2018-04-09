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
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);
    let (mut x, mut y, mut bomb_area) = (x0, y0, (0, w - 1, 0, h - 1));

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        bomb_area = match bomb_dir.as_ref() {
            "U"  => (x, x, bomb_area.2, y - 1),
            "D"  => (x, x, y + 1, bomb_area.3),
            "L"  => (bomb_area.0, x - 1, y, y),
            "R"  => (x + 1, bomb_area.1, y, y),
            "UR" => (x + 1, bomb_area.1, bomb_area.2, y - 1),
            "DR" => (x + 1, bomb_area.1, y + 1, bomb_area.3),
            "DL" => (bomb_area.0, x - 1, y + 1, bomb_area.3),
            "UL" => (bomb_area.0, x - 1, bomb_area.2, y - 1),
            _ => panic!("unexpected...")
        };
        x = (bomb_area.0 + bomb_area.1) / 2;
        y = (bomb_area.2 + bomb_area.3) / 2;
        println!("{} {}", x, y);
    }
}