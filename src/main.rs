use std::{fs::File, io::{prelude::*, BufReader}, path::Path, env};

mod fsm;
use crate::fsm::FSM;
use std::error::Error;

fn read_fsm_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("FSM file not found!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn read_tape_file(filename: impl AsRef<Path>) -> Vec<char> {
    let file = File::open(filename).expect("Input file not found!");
    let buf = BufReader::new(file);
    // Read the first line
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let first_line: Vec<char> = lines[0].chars().collect();
    first_line
}

fn read_line(line: &String) -> Result<(i32, char, char, i32), Box<dyn Error>> {
    let line: Vec<&str> = line.split_whitespace().collect();

    let state = line[0].parse::<i32>()?;
    let input = line[1].parse::<char>()?;
    let output = line[2].parse::<char>()?;
    let next= line[3].parse::<i32>()?;

    Ok((state, input, output, next))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("No file specified.")
    }

    // Read everything from file
    let raw_lines = read_fsm_file(&args[1]);
    let input = read_tape_file(&args[2]);

    let mut fsm: FSM<i32, char> = fsm::FSM::new();

    // Add lines to FSM
    for line in raw_lines.iter() {
        if line.is_empty() {
            continue
        }
        let (state, input, output, next) = match read_line(line) {
            Err(_) => panic!("Bad description"),
            Ok(x) => x,
        };
        match fsm.add_line(state, input, output, next) {
            Err(_) => panic!("Error compiling FSM"),
            Ok(x) => x,
        }
    }

    //println!("{:?}", fsm);

    if !fsm.validate() {
        panic!("Bad description")
    }

    for i in input.iter() {
        //println!("In: {}", i);
        let out = match fsm.next_state(*i) {
            Ok(x) => x,
            Err(_) => panic!("Bad input")
        };
        //println!("{:?}", fsm);
        print!("{}", out)
    }
}