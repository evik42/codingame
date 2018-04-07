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
    let surface_n = read_single_input!(i32); // the number of points used to draw the surface of Mars.
    let (tx1, tx2, th) = read_surface_data(surface_n);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs : Vec<_> = input_line.trim().split_whitespace().map(|s| -> i32 {s.parse().unwrap()}).collect();
        let y = inputs[1];
        let v_speed = inputs[3]; // the vertical speed (in m/s), can be negative.
        let freefall = freefall_time(y, th, v_speed);

        println!("0 {}", if freefall < 1 {4} else {0}); // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1), power is the desired thrust power (0 to 4).
    }
}

fn read_surface_data(n: i32) -> (i32, i32, i32) {
    let (mut lastx, mut lasth) = (-1, -1);
    let mut target = (0, 0, 0);
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input : Vec<_> = input.split(" ").collect();
        let x = parse_input!(input[0], i32);
        let h = parse_input!(input[1], i32);
        if h == lasth {
            target = (lastx, x, h);
        }
        lastx = x;
        lasth = h;
    }
    target
}

fn freefall_time(h: i32, th: i32, speed: i32) -> i32 {
    let road = (h - th) as f32;
    let v = - speed as f32;
    let a : f32 = 13.771521 / 0.598 + 1.8555;
    let b : f32 = (7.422 * v + 38.097126) / 0.598 + 2.5665 + v + 5.5665;
    let c : f32 = (v * v + 10.266 * v - 1573.652311) / 0.598 - road + 3.0 * v + 7.6995; // +3v + 7.6995
    let disc = b * b - 4.0 * a * c;
    let disc = disc.sqrt();
    let x = (- b + disc) / (2.0 * a);
    let x = x.floor();
    x as i32
}
