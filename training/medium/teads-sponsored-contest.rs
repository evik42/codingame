use std::io;
use std::collections::HashMap;
use std::collections::BTreeSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

type Graph = HashMap<i32, BTreeSet<i32>>;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize); // the number of adjacency relations
    let mut g: Graph = HashMap::with_capacity(n);
    for _i in 0..n {
        let (x, y) = read_edge();
        add_edge(x, y, &mut g);
    }
    let mut d = 0;
    while g.len() > 1 {
        let mut removes: Vec<(i32, i32)> = Vec::with_capacity(g.len());
        for (&k, v) in &g {
            if v.len() == 1 {
                removes.push((k, v.iter().next().unwrap().clone()));
            }
        }
        for (x, y) in removes {
            g.remove(&x);
            let so = g.get_mut(&y);
            if so.is_some() {
                let mut s = so.unwrap();
                s.remove(&x);
            }
        }
        d += 1;
    }

    println!("{}", d);
}

fn read_edge() -> (i32, i32) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let xi = parse_input!(inputs[0], i32); // the ID of a person which is adjacent to yi
    let yi = parse_input!(inputs[1], i32); // the ID of a person which is adjacent to xi
    (xi, yi)
}

fn add_edge(x: i32, y: i32, g: &mut Graph) {
    fn add_link(x: i32, y: i32, g: &mut Graph) {
        let s = g.entry(x).or_insert_with(BTreeSet::new);
        s.insert(y);
    }
    add_link(x, y, g);
    add_link(y, x, g);
}
