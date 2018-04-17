use std::io::*;
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut buf = BufReader::new(stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap();
    let mut numeral_to_digit = HashMap::with_capacity(20);
    let mut digit_to_numeral = HashMap::with_capacity(20);
    let input_line = read_line();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);
    let size = h * l;
    let mut ns = Vec::with_capacity(20);
    for _ in 0..20 {
        ns.push(String::with_capacity(size));
    }
    for _ in 0..h {
        for (i, c) in read_line().trim().chars().enumerate() {
            let n = i / l;
            ns[n].push(c);
        }
    }
    for (i, n) in ns.into_iter().enumerate() {
        let digit = std::char::from_digit(i as u32, 20).unwrap();
        numeral_to_digit.insert(n.clone(), digit);
        digit_to_numeral.insert(i, n);
    }
    let s1 = parse_input!(read_line(), usize);
    let ns1 = s1 / h;
    let mut n1m = String::new();
    for _i in 0..ns1 {
        let mut s = String::with_capacity(size);
        for _j in 0..h {
            s.push_str(read_line().trim());
        }
        n1m.push(*numeral_to_digit.get(&s).unwrap());
    }
    let n1 = u64::from_str_radix(&n1m, 20).unwrap();
    let s2 = parse_input!(read_line().trim(), usize);
    let ns2 = s2 / h;
    let mut n2m = String::new();
    for _i in 0..ns2 {
        let mut s = String::with_capacity(size);
        for _j in 0..h {
            s.push_str(read_line().trim());
        }
        n2m.push(*numeral_to_digit.get(&s).unwrap());
    }
    let n2 = u64::from_str_radix(&n2m, 20).unwrap();
    let mut res = match read_line().trim() {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "*" => n1 * n2,
        "/" => n1 / n2,
        _ => panic!("unexpected operator"),
    } as usize;
    let mut resm = Vec::new();
    if res == 0 {
        resm.push(digit_to_numeral.get(&0).unwrap());
    }
    while res > 0 {
        let digit = res % 20;
        resm.push(digit_to_numeral.get(&digit).unwrap());
        res = res / 20;
    }
    for numeral in resm.iter().rev() {
        for (i, c) in numeral.chars().enumerate() {
            if i > 0 && i % l == 0 {
                println!("");
            }
            print!("{}", c);
        }
        println!("");
    }
}
