use std::io;
use std::collections::HashMap;


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Copy,Clone)]
struct Node {
    coords: (i32, i32),
    right:  (i32, i32),
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis
    let mut bottoms: HashMap<i32, Node> =  HashMap::with_capacity(width as usize);
    for i in 0..height {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut line = input_line.trim_right().to_string(); // width characters, each either 0 or .
        let mut last: Option<Node> = None;
        for j in (0..width).rev() {
            match line.pop().unwrap() {
                '0' => {
                    let coords = (j, i);
                    let right  = if last.is_some() {last.unwrap().coords} else {(-1, -1)};
                    let n = Node {
                        coords: coords,
                        right: right,
                    };
                    let top = bottoms.insert(j, n);
                    if top.is_some() {
                        print_node(top.unwrap(), coords)
                    }
                    last = Some(n);
                },
                _ => {},
            }
        }
    }

    for (_, n) in bottoms {
        print_node(n, (-1, -1));
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    fn print_node(n: Node, bot: (i32, i32)) {
        let (cx, cy) = n.coords;
        let (rx, ry) = n.right;
        let (bx, by) = bot;
        // Three coordinates: a node, its right neighbor, its bottom neighbor
        println!("{} {} {} {} {} {}", cx, cy, rx, ry, bx, by);
    }
}
