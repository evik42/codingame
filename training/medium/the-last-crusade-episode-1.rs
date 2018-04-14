use std::io;
use std::collections::HashMap;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(PartialEq, Eq, Hash)]
enum Entry {
    Top,
    Left,
    Right,
}

type ParseEntryError = ();

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TOP" => Ok(Entry::Top),
            "LEFT" => Ok(Entry::Left),
            "RIGHT" => Ok(Entry::Right),
            _ => Err(())
        }
    }
}

enum Exit {
    Left,
    Right,
    Down,
}

#[derive(Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn go(&self, e: &Exit) -> Coords {
        let mut res = self.clone();
        match e {
            &Exit::Left  => res.x -= 1,
            &Exit::Right => res.x += 1,
            &Exit::Down  => res.y += 1,
        }
        res
    }
}

type RoomType = i32;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize); // number of columns.
    let h = parse_input!(inputs[1], usize); // number of rows.
    let mut map = Vec::with_capacity(h);
    for i in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.split(" ").map(|s| { parse_input!(s, i32) }).collect::<Vec<RoomType>>(); // represents a line in the grid and contains W integers. Each integer represents one room of a given type.
        map.push(line);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let ex = parse_input!(input_line, i32); // the coordinate along the X axis of the exit (not useful for this first mission, but must be read).

    let rooms = create_rooms();
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let xi = parse_input!(inputs[0], i32);
        let yi = parse_input!(inputs[1], i32);
        let pos = parse_input!(inputs[2], Entry);

        let coords = Coords { x: xi, y: yi };
        let room_type = map.get(yi as usize).unwrap().get(xi as usize).unwrap();
        let next_step = coords.go(rooms.get(&room_type).unwrap().get(&pos).unwrap());

        // One line containing the X Y coordinates of the room in which you believe Indy will be on the next turn.
        println!("{} {}", next_step.x, next_step.y);
    }
}

fn create_rooms() -> HashMap<RoomType, HashMap<Entry, Exit>> {
    let mut m = HashMap::with_capacity(14);
    let mut r = HashMap::new();
    m.insert(0, r);
    r = HashMap::with_capacity(3);
    r.insert(Entry::Top, Exit::Down);
    r.insert(Entry::Left, Exit::Down);
    r.insert(Entry::Right, Exit::Down);
    m.insert(1, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Left, Exit::Right);
    r.insert(Entry::Right, Exit::Left);
    m.insert(2, r);
    r = HashMap::with_capacity(1);
    r.insert(Entry::Top, Exit::Down);
    m.insert(3, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Top, Exit::Left);
    r.insert(Entry::Right, Exit::Down);
    m.insert(4, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Top, Exit::Right);
    r.insert(Entry::Left, Exit::Down);
    m.insert(5, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Left, Exit::Right);
    r.insert(Entry::Right, Exit::Left);
    m.insert(6, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Top, Exit::Down);
    r.insert(Entry::Right, Exit::Down);
    m.insert(7, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Left, Exit::Down);
    r.insert(Entry::Right, Exit::Down);
    m.insert(8, r);
    r = HashMap::with_capacity(2);
    r.insert(Entry::Top, Exit::Down);
    r.insert(Entry::Left, Exit::Down);
    m.insert(9, r);
    r = HashMap::with_capacity(1);
    r.insert(Entry::Top, Exit::Left);
    m.insert(10, r);
    r = HashMap::with_capacity(1);
    r.insert(Entry::Top, Exit::Right);
    m.insert(11, r);
    r = HashMap::with_capacity(1);
    r.insert(Entry::Right, Exit::Down);
    m.insert(12, r);
    r = HashMap::with_capacity(1);
    r.insert(Entry::Left, Exit::Down);
    m.insert(13, r);
    m
}
