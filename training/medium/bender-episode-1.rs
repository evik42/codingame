use std::fmt;
use std::io;
use std::ops::Not;
use std::collections::HashSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    South,
    East,
    North,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Direction::South => "SOUTH",
            Direction::East  => "EAST",
            Direction::North => "NORTH",
            Direction::West  => "WEST",
        })
    }
}

impl Direction {
    fn step(&self, (x, y): Coords) -> Coords {
        match *self {
            Direction::South => (x + 1, y),
            Direction::North => (x - 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Breaker {
    True,
    False,
}

impl Not for Breaker {
    type Output = Breaker;

    fn not(self) -> Breaker {
        match self {
            Breaker::True => Breaker::False,
            Breaker::False => Breaker::True,
        }
    }
}

impl Breaker {
    fn is_valid_move(&self, &c: &char) -> bool {
        (*self == Breaker::True && c == 'X') || (c != '#' && c != 'X')
    }
}

struct Teleport {
    a: Coords,
    b: Coords,
}

impl Teleport {
    fn port(&self, i: Coords) -> Coords {
        if i == self.a {self.b} else {self.a}
    }
}

type Coords = (usize, usize);
type Map = Vec<Vec<char>>;

struct State {
    map: Map,
    teleport: Option<Teleport>,
    priority: Vec<Direction>,
    breaker: Breaker,
    coords: Coords,
    direction: Direction,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut s = read_state();
    let mut steps = Vec::new();
    let is_loop;
    let mut visits = HashSet::new();
    loop {
        let ns = next_step(&s);
        s.direction = ns;
        steps.push(ns);
        let (x, y) = ns.step(s.coords);
        s.coords = (x, y);
        let ref mut c = s.map[x][y];
        if *c == '$' { is_loop = false; break; };
        let v = ((x, y), ns, s.breaker);
        if visits.contains(&v) { is_loop = true; break; };
        visits.insert(v);
        if *c == 'I' { s.priority.reverse(); };
        if *c == 'B' { s.breaker = ! s.breaker; };
        if *c == 'T' { s.coords = s.teleport.as_ref().unwrap().port(s.coords); };
        if *c == 'X' { *c = ' '; visits.clear(); };
    }
    if is_loop {
        println!("LOOP");
    } else {
        for d in steps {
            println!("{}", d);
        }
    }
}

fn next_step(s: &State) -> Direction {
    let (x, y) = s.coords;
    let c = s.map[x][y];
    match c {
        'S' => Direction::South,
        'E' => Direction::East,
        'N' => Direction::North,
        'W' => Direction::West,
        _ => {
            let ds = valid_directions(s);
            if ds.contains(&s.direction) {
                s.direction.clone()
            } else {
                for d in &s.priority {
                    if ds.contains(d) {
                        return d.clone();
                    }
                }
                Direction::South // should never reach this
            }
        }
    }
}

fn valid_directions(s: &State) -> HashSet<Direction> {
    let mut ds = HashSet::new();
    let (x, y) = s.coords;
    if s.breaker.is_valid_move(&s.map[x + 1][y]) {
        ds.insert(Direction::South);
    }
    if s.breaker.is_valid_move(&s.map[x - 1][y]) {
        ds.insert(Direction::North);
    }
    if s.breaker.is_valid_move(&s.map[x][y + 1]) {
        ds.insert(Direction::East);
    }
    if s.breaker.is_valid_move(&s.map[x][y - 1]) {
        ds.insert(Direction::West);
    }
    ds
}

fn read_state() -> State {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], usize);
    let c = parse_input!(inputs[1], i32);
    let mut map: Map = Vec::with_capacity(l);
    let mut coords: Coords = (0, 0);
    let mut teleports = Vec::new();
    for i in 0..l {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right().to_string().chars().collect::<Vec<_>>();
        if coords == (0, 0) {
            let mut start = row.iter().enumerate().filter(|&(_i, c)| { *c == '@' }).collect::<Vec<_>>();
            if !start.is_empty() {
                let (j, _) = start.pop().unwrap();
                coords = (i, j as usize);
            }
        }
        {
            let mut ts = row.iter().enumerate().filter(|&(_i, c)| { *c == 'T' }).collect::<Vec<_>>();
            while ! ts.is_empty() {
                let (j, _) = ts.pop().unwrap();
                teleports.push((i, j as usize));
            }
        }
        map.push(row);
    }

    let teleport = if teleports.is_empty() {None} else {
        let a = teleports.pop().unwrap().to_owned();
        let b = teleports.pop().unwrap().to_owned();
        Some(Teleport { a, b })
    };
    let priority = vec![Direction::South, Direction::East, Direction::North, Direction::West];
    let breaker = Breaker::False;

    State { map, teleport, direction: *priority.first().unwrap(), priority, breaker, coords, }
}
