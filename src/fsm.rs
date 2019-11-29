use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::str::FromStr;

// Represents a Finite State Machine.
// S: type for states
// IO: type for input/output
// usize: indices

pub(crate) struct FSM <S: PartialEq, IO: Copy + Eq + Hash + FromStr > {
    states: Vec<S>,
    current_state: usize,
    state_mapping: HashMap<(usize, IO), usize>,
    output_mapping: HashMap<(usize, IO), IO>
}

impl<S: FromStr + PartialEq + Copy, IO: Copy + Eq + Hash + FromStr> FSM<S, IO> {
    pub fn new() -> FSM<S, IO> {
        FSM {
            states: vec![],
            current_state: 0,
            state_mapping: HashMap::new(),
            output_mapping: HashMap::new()
        }
    }

    pub fn add_line(&mut self, state: S, input: IO, output: IO, next: S)
        -> Result<(), Box<dyn Error>> {

        // Determine usize of state and next
        let state_u: usize = self.get_or_add_state(state);
        let next_u: usize = self.get_or_add_state(next);

        self.state_mapping.insert((state_u, input), next_u);
        self.output_mapping.insert((state_u, input.clone()), output);

        Ok(())

    }

    pub fn next_state(&mut self, input: IO) -> Result<IO, Box<dyn Error>> {
        let out: IO = *self.output_mapping.get(&(self.current_state, input))
            .expect("Bad input");
        self.current_state = *self.state_mapping.get(&(self.current_state, input))
            .expect("Bad input");
        Ok(out)
    }

    pub fn validate(&mut self) -> bool {
        // Are there any states that do not accept the entirety of the input set?
        // Use initial state as the base
        let initial_state_inputs = self.get_accepted_inputs(0);
        // For each state
        for i in 1..self.states.len() {
            let accepted = self.get_accepted_inputs(i);
            if accepted != initial_state_inputs {
                return false;
            }
        }
        return true;
    }

    fn get_accepted_inputs(&self, state: usize) -> HashSet<&IO> {
        let mut accepted: HashSet<&IO> = HashSet::new();
        for (key, _value) in &self.state_mapping {
            if key.0 == state {
                accepted.insert(&key.1);
            }
        }
        accepted
    }

    fn get_or_add_state(&mut self, state: S) -> usize {
        match self.states.iter().position(|i| *i == state) {
            Some(x) => x,
            None => {
                self.states.push(state.clone());
                self.get_or_add_state(state)
            }
        }
    }
}