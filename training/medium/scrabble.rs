use std::io;
use std::collections::BTreeSet;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Word {
    word: String,
    score: i32,
    letters: BTreeSet<String>,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut words = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let word = input_line.trim_right().to_string();
        let mut score = 0;
        let mut letters = BTreeSet::new();
        for c in word.chars() {
            let mut count = 1;
            while ! letters.insert(format!("{}{}", count, c)) {
                count += 1
            }
            score += point_value(&c);
        }
        words.push(Word { word, score, letters });
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let w = input_line.trim_right().to_string();
    let mut letters = BTreeSet::new();
    for c in w.chars() {
        let mut count = 1;
        while ! letters.insert(format!("{}{}", count, c)) {
            count += 1
        }
    }
    // reverse is needed as we need to find out the first word that matches with highest score and max_by_key returns the last
    let w = words.iter().filter(|&w| { letters.is_superset(&w.letters) }).rev().max_by_key(|&w| { w.score }).unwrap();

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", w.word);
}

fn point_value(&c: &char) -> i32 {
    match c {
        'q' | 'z' => 10,
        'j' | 'x' => 8,
        'k' => 5,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'b' | 'c' | 'm' | 'p' => 3,
        'd' | 'g' => 2,
        _ => 1,
    }
}
