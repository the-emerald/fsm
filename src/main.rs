use std::{fs::File, io::{prelude::*, BufReader}, path::Path, env};
use std::str::{FromStr};
use crate::state::State;
use std::collections::HashMap;
use std::error::Error;

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

fn add_line_to_fsm(fsm: &mut HashMap<i32, State>, line: &str) -> Result<(), Box<dyn Error>> {
    let line: Vec<&str> = line.split_whitespace().collect();
    // State, input, output, nState
    let state_id = line[0].parse::<i32>()?;
    let input = line[1].parse::<char>()?;
    let output = line[2].parse::<char>()?;
    let next_state_id = line[3].parse::<i32>()?;

    println!("{} {} {} {}", state_id, input, output, next_state_id);
    let state = fsm.entry(state_id).or_insert(
        State::new(state_id)
    );

    // Add input->output to state
    state.update_state_output(input, output);

    let next_state = fsm.entry(next_state_id).or_insert(
        State::new(next_state_id)
    );

    // Add input->nState to state
    state.update_state_state(input, next_state.clone());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("No file specified.")
    }

    let mut fsm: HashMap<i32, State> = HashMap::new();

    // Read everything from file
    let raw_lines = read_lines(&args[1]);
    let input = read_tape(&args[2]);
    //println!("{:?}", input);

    // Use the first line to make an initial state
    let initial_state = state::State::from_str(&raw_lines[0]).unwrap(); // use match

    // Use the initial state to make FSM
    //let mut fsm_old = fsm::FSM::new(Some(initial_state));

    // Iterate over the rest to generate the FSM
    for line in raw_lines.iter() {
        match add_line_to_fsm(&mut fsm, &line) {
        //match fsm_old.add(&line) {
            Err(_) => panic!("Bad description"),
            Ok(x) => x,
        }
    }

//    let mut current_state: State = match fsm_old.get_initial_state().clone() {
//        None => panic!("No initial state set"),
//        Some(x) => x
//    };
//
//    for i in input.iter() {
//        println!("{}", i);
//        println!("{}", match current_state.get_output(i) {
//            Some(x) => x,
//            None => panic!("Bad input")
//        });
//        current_state = match current_state.get_next_state(i) {
//            None => panic!("Bad input"),
//            Some(x) => x.clone(),
//        }
//    }
}
