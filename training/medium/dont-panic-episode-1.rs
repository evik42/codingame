use std::io;
use std::collections::HashMap;

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
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let nb_floors = parse_input!(inputs[0], i32); // number of floors
    let width = parse_input!(inputs[1], i32); // width of the area
    let nb_rounds = parse_input!(inputs[2], i32); // maximum number of rounds
    let exit_floor = parse_input!(inputs[3], i32); // floor on which the exit is found
    let exit_pos = parse_input!(inputs[4], i32); // position of the exit on its floor
    let nb_total_clones = parse_input!(inputs[5], i32); // number of generated clones
    let nb_additional_elevators = parse_input!(inputs[6], i32); // ignore (always zero)
    let nb_elevators = parse_input!(inputs[7], i32); // number of elevators
    let mut elevator_map = HashMap::with_capacity(nb_floors as usize);
    for i in 0..nb_elevators as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let elevator_floor = parse_input!(inputs[0], i32); // floor on which this elevator is found
        let elevator_pos = parse_input!(inputs[1], i32); // position of the elevator on its floor
        elevator_map.insert(elevator_floor, elevator_pos);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let clone_floor = parse_input!(inputs[0], i32); // floor of the leading clone
        let clone_pos = parse_input!(inputs[1], i32); // position of the leading clone on its floor
        let direction = inputs[2].trim().to_string(); // direction of the leading clone: LEFT or RIGHT
        let action = if direction == "NONE" {"WAIT"} else {
            let target_pos = if clone_floor == exit_floor {exit_pos} else {*elevator_map.get(&clone_floor).unwrap()};
            let target_dir = clone_pos - target_pos;
            if (target_dir <= 0 && direction == "RIGHT") || (target_dir >= 0 && direction == "LEFT") {"WAIT"} else {"BLOCK"}
        };
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", action); // action: WAIT or BLOCK
    }
}
