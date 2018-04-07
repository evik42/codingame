use std::io;
use std::u32;

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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_right().to_string();
    let mut bits = Vec::with_capacity(message.len() * 7);
    for c in message.chars().rev() {
        let mut ascii = u32::from(c);
        for _ in 0..7 {
            let bit = if ascii % 2 == 0 {'0'} else {'1'};
            bits.push(bit);
            ascii = ascii >> 1;
        }
    }
    let mut cur_bit = '-';
    let mut cur_num = 0;
    while bits.len() > 0 {
        let bit = bits.pop().unwrap();
        if bit == cur_bit {
            cur_num += 1;
        } else {
            if cur_num > 0 {
                print_bits(cur_bit, cur_num);
                print!(" ");
            }
            cur_bit = bit;
            cur_num = 1;
        }
    }
    print_bits(cur_bit, cur_num);
    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("");
}

fn print_bits(bit: char, num: u8) {
    print!("{} ", if bit == '1' {"0"} else {"00"});
    for _ in 0..num {
        print!("0");
    }
}
