use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone)]
pub struct State<'a>{
    id: i32,
    state_to_state: HashMap<char, &'a State<'a>>,
    state_to_output: HashMap<char, char>,
}

impl<'a> State<'a> {
    pub fn new(id: i32) -> State<'a> {
        State {
            id,
            state_to_state: Default::default(),
            state_to_output: Default::default()
        }
    }
    pub fn update_state_state(&mut self, input: char, next: &'a State<'a>) {
        self.state_to_state.insert(input, next);
    }

    pub fn update_state_output(&mut self, input: char, next: char) {
        self.state_to_output.insert(input, next);
    }

    pub fn get_output(&self, input: &char) -> Option<&char> {
        self.state_to_output.get(input)
    }

    pub fn get_next_state(&self, input: &char) -> Option<&&State> {
        self.state_to_state.get(input)
    }
}

impl<'a> FromStr for State<'a> {
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

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for State<'a> {}