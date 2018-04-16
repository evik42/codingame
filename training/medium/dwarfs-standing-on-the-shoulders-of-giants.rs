use std::io;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

type Person = i32;
type Influences = BTreeSet<i32>;
type Relations = BTreeMap<Person, Influences>;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of relationships of influence
    let mut people = BTreeMap::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32); // a relationship of influence between two people (x influences y)
        let y = parse_input!(inputs[1], i32);
        {
            let influences = people.entry(x).or_insert_with(BTreeSet::new);
            influences.insert(y);
        }
        people.entry(y).or_insert_with(BTreeSet::new);
    }

    let mut length = 0;

    while ! people.is_empty() {
        let (leaves, tree): (Relations, Relations) = people.into_iter().partition(|r| { r.1.is_empty() });
        people = tree;
        for (p, _) in leaves {
            for influences in people.values_mut() {
                influences.remove(&p);
            }
        }
        length += 1;
    }
    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");


    // The number of people involved in the longest succession of influences
    println!("{}", length);
}
