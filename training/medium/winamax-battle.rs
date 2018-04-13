use std::io;
use std::cmp::min;
use std::collections::VecDeque;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

type Deck = VecDeque<i32>;

enum Result {
    PAT,
    PlayerOne(i32),
    PlayerTwo(i32)
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut deck1 = read_deck();
    let mut deck2 = read_deck();
    match war(&mut deck1, &mut deck2) {
        Result::PAT => println!("PAT"),
        Result::PlayerOne(r) => println!("1 {}", r),
        Result::PlayerTwo(r) => println!("2 {}", r)
    };
}

fn parse_card(card: &str) -> i32 {
    match card {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        c   => parse_input!(c, i32),
    }
}

fn read_deck() -> Deck {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);

    let mut deck = VecDeque::with_capacity(n);
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut card = input_line.trim().to_string();
        card.pop();
        let rank = parse_card(&card);
        deck.push_back(rank);
    }
    deck
}

fn war(deck1: &mut Deck, deck2: &mut Deck) -> Result {
    let mut rounds = 1;
    let mut res = None;
    let mut table1: Deck = VecDeque::with_capacity(deck1.len());
    let mut table2: Deck = VecDeque::with_capacity(deck2.len());
    while res.is_none() {
        if deck1.is_empty() || deck2.is_empty() {
            res = Some(Result::PAT);
        } else {
            let card1 = deck1.pop_front().unwrap();
            let card2 = deck2.pop_front().unwrap();
            table1.push_back(card1);
            table2.push_back(card2);
            if card1 > card2 {
                if deck2.is_empty() {
                    res = Some(Result::PlayerOne(rounds));
                } else {
                    deck1.append(&mut table1);
                    deck1.append(&mut table2);
                    rounds += 1;
                }
            } else if card1 < card2 {
                if deck1.is_empty() {
                    res = Some(Result::PlayerTwo(rounds));
                } else {
                    deck2.append(&mut table1);
                    deck2.append(&mut table2);
                    rounds += 1;
                }
            } else {
                let d1 = min(3, deck1.len());
                table1.extend(deck1.drain(..d1).into_iter());
                let d2 = min(3, deck2.len());
                table2.extend(deck2.drain(..d2).into_iter());
            }
        }
    }
    res.unwrap()
}
