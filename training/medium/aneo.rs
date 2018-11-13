use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Light {
  distance: i32,
  duration: i32,
}

fn valid_speed(lights: &Vec<Light>, speed: i32) -> bool {
  for l in lights {
    let t = l.distance / speed;
    if (t / l.duration) % 2 != 0 { return false; }
  }
  true
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let max_speed = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let light_count = parse_input!(input_line, i32);
    let mut lights: Vec<Light> = vec![];
    for i in 0..light_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let dist = parse_input!(inputs[0], i32);
        let duration = parse_input!(inputs[1], i32);
        let distance = (dist as f32 * 3.6).floor() as i32;
        lights.push(Light { distance, duration });
    }

    let mut speed = max_speed;
    while speed > 0 && !valid_speed(&lights, speed) {
      speed -= 1;
    }

    println!("{}", speed);
}
