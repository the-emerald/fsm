use std::collections::HashMap;
use crate::state::State;
use std::error::Error;

pub struct FSM {
    initial_state: Option<State>,
    id_to_state: HashMap<i32, State>,
}

impl FSM {
    pub(crate) fn new() -> FSM {
        FSM {
            initial_state: None,
            id_to_state: HashMap::new(),
        }
    }

    // This method adds a line to the FSM
    pub fn add(&mut self, line: &str) -> Result<(), Box<dyn Error>>{
        // Decompose into a vector
        let line: Vec<&str> = line.split_whitespace().collect();

        // State, input, output, nState
        let state_id = line[0].parse::<i32>()?;
        let input = line[1].parse::<char>()?;
        let output = line[2].parse::<char>()?;
        let next_state_id = line[3].parse::<i32>()?;

        // Use id_to_state to find state
        let state = self.id_to_state.entry(state_id).or_insert(
            State::new(state_id)
        );


        // Use id_to_state to find nState
        let next_state = self.id_to_state.entry(next_state_id).or_insert(
            State::new(next_state_id)
        );

        // Add input->nState to state
        state.update_state_state(input, next_state);

        // Add input->output to state
        state.update_state_output(input, output);
        Ok(())
    }
}