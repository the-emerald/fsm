use std::{fs::File, io::{prelude::*, BufReader}, path::Path, env};
use std::str::{FromStr};
use crate::state::State;

mod fsm;
mod state;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("FSM file not found!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn read_tape(filename: impl AsRef<Path>) -> Vec<char> {
    let file = File::open(filename).expect("Input file not found!");
    let buf = BufReader::new(file);
    // Read the first line
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let first_line: Vec<char> = lines[0].chars().collect();
    first_line
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("No file specified.")
    }

    // Read everything from file
    let raw_lines = read_lines(&args[1]);
    let input = read_tape(&args[2]);
    //println!("{:?}", input);

    println!("{}", &raw_lines[0]);
    // Use the first line to make an initial state
    let initial_state = state::State::from_str(&raw_lines[0]).unwrap();

    // Use the initial state to make FSM
    let mut fsm = fsm::FSM::new(Some(initial_state));

    // Iterate over the rest to generate the FSM
    for line in raw_lines.iter().skip(1) {
        match fsm.add(&line) {
            Err(_) => panic!("Bad description"),
            Ok(x) => x,
        }
        println!("{}", &line);
    }
}
