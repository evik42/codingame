use std::io;
use std::collections::HashSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

type Height = i32;
type Coords = (usize, usize);
type Square = (Coords, Height);
type Area = HashSet<Square>;
type Row = Vec<Height>;
type Map = Vec<Row>;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    let mut map: Map = Vec::with_capacity(h);
    for i in 0..h {
        let mut row: Row = Vec::with_capacity(w);
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for j in inputs.split(" ") {
            let height = parse_input!(j, i32);
            row.push(height);
        }
        map.push(row);
    }
    let mut drains = 0;
    let mut s = find_lowest_square(&map, w, h);
    while s.1 < 10001 {
        let mut area: Area = HashSet::new();
        area.insert(s);
        let mut last_size = 0;
        while area.len() > last_size {
            last_size = area.len();
            extend_area(&map, &mut area);
        }
        drains += 1;
        for ((x, y), _) in area {
            set_height(&mut map, (x, y), 10001);
        }
        s = find_lowest_square(&map, w, h);
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", drains);
}

fn find_lowest_square(map: &Map, w: usize, h: usize) -> Square {
    let mut coords = (101, 101);
    let mut min = 10001;
    for i in 0..h {
        let row = map.get(i).unwrap();
        for j in 0..w {
            let &height = row.get(j).unwrap();
            if height < min {
                min = height;
                coords = (i, j);
            }
        }
    }
    (coords, min)
}

fn get_height(map: &Map, (x, y): Coords) -> &Height {
    map.get(x).unwrap().get(y).unwrap()
}

fn set_height(map: &mut Map, (x, y): Coords, height: Height) {
    let row = map.get_mut(x).unwrap();
    if let Some(h) = row.get_mut(y) {
        *h = height;
    }
}

fn extend_area(map: &Map, area: &mut Area) {
    let mut extension: Area = HashSet::new();
    for &((x, y), height) in area.iter() {
        if x > 0 {
            if let Some(s) = check_square(map, (x - 1, y), height) {
                extension.insert(s);
            }
        }
        if x < map.len() - 1 {
            if let Some(s) = check_square(map, (x + 1, y), height) {
                extension.insert(s);
            }
        }
        if y > 0 {
            if let Some(s) = check_square(map, (x, y - 1), height) {
                extension.insert(s);
            }
        }
        if y < map.first().unwrap().len() - 1 {
            if let Some(s) = check_square(map, (x, y + 1), height) {
                extension.insert(s);
            }
        }
    }
    for s in extension {
        area.insert(s);
    }
}

fn check_square(map: &Map, (x, y): Coords, height: i32) -> Option<Square> {
    let &h = get_height(map, (x, y));
    if h < 10001 && h >= height {Some(((x, y), h))} else {None}
}
