use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
pub struct State{
    id: i32,
    state_to_state: HashMap<char, State>,
    state_to_output: HashMap<char, char>,
}

impl State {
    pub fn new(id: i32) -> State {
        State {
            id,
            state_to_state: Default::default(),
            state_to_output: Default::default()
        }
    }
    pub fn update_state_state(&mut self, input: char, next: State) {
        self.state_to_state.insert(input, next);
    }

    pub fn update_state_output(&mut self, input: char, next: char) {
        self.state_to_output.insert(input, next);
    }
}

impl FromStr for State {
    type Err = ParseIntError;

    // Make a new State from one line
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = s.split_whitespace().collect();

        let id_fromstr = line[0].parse::<i32>()?;

        Ok(State {
            id: id_fromstr,
            state_to_state: Default::default(),
            state_to_output: Default::default()
        })
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for State {}