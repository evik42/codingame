use std::io::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut buf = BufReader::new(stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap();

    let n = parse_input!(read_line(), i32);
    let mut c = parse_input!(read_line(), i32);
    let mut budgets = (0..n).map(|_| parse_input!(read_line(), i32)).collect::<Vec<_>>();
    budgets.sort_unstable();
    let mut contributions = Vec::new();

    let mut l = budgets.len() as i32;
    for b in budgets.into_iter() {
        let avg = c / l;
        let x = if b <= avg {b} else {avg};
        contributions.push(x);
        c -= x;
        l -= 1;
    }

    if c > 0 {
        println!("IMPOSSIBLE");
    } else {
        for b in contributions {
            println!("{}", b);
        }
    }
}
