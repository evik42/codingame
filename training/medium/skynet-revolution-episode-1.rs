use std::io;
use std::collections::BTreeSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

type Node = i32;
type Graph = Vec<BTreeSet<Node>>;
type Path = Vec<Node>;
type Paths = Vec<Path>;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], usize); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], usize); // the number of links
    let e = parse_input!(inputs[2], usize); // the number of exit gateways
    let mut links = Vec::with_capacity(n);
    for i in 0..n {
        links.push(BTreeSet::new());
    }
    for i in 0..l {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32); // N1 and N2 defines a link between these nodes
        let n2 = parse_input!(inputs[1], i32);
        add_link(n1, n2, &mut links);
        add_link(n2, n1, &mut links);
    }
    let mut gates = BTreeSet::new();
    for i in 0..e {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let ei = parse_input!(input_line, i32); // the index of a gateway node
        gates.insert(ei);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, i32); // The index of the node on which the Skynet agent is positioned this turn
        let mut p = find_shortest_path(si, &links, &gates);
        let n1 = p.pop().unwrap();
        let n2 = p.pop().unwrap();
        remove_link(n1, n2, &mut links);
        remove_link(n2, n1, &mut links);
        println!("{} {}", n1, n2);
    }
}

fn add_link(n1: i32, n2: i32, links: &mut Graph) {
    let s = links.get_mut(n1 as usize).unwrap();
    s.insert(n2);
}

fn remove_link(n1: i32, n2: i32, links: &mut Graph) {
    let s = links.get_mut(n1 as usize).unwrap();
    s.remove(&n2);
}

fn find_shortest_path(agent: Node, links: &Graph, gates: &BTreeSet<Node>) -> Path {
    fn go(mut paths: Paths, visited: &mut BTreeSet<Node>, links: &Graph, gates: &BTreeSet<Node>) -> Path {
        let mut new_paths: Paths = Vec::new();
        while !paths.is_empty() {
            let v = paths.pop().unwrap();
            let &n = v.last().unwrap();
            for &next in links.get(n as usize).unwrap() {
                if !visited.contains(&next) {
                    visited.insert(next);
                    let mut nv = v.clone();
                    nv.push(next);
                    if gates.contains(&next) {
                        return nv;
                    }
                    new_paths.push(nv);
                };
            }
        }
        go(new_paths, visited, links, gates)
    }
    let mut visited = BTreeSet::new();
    visited.insert(agent);
    go(vec![vec![agent]], &mut visited, links, gates)
}
