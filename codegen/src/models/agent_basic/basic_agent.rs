use crate::models::general::llm::Message;
use crate::models::agent_basic::basic_traits::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([])
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&mut self) -> &String {
        &self.objective
    }

    fn get_position(&mut self) -> &String {
        &self.position
    }

    fn get_state(&mut self) -> &AgentState {
        &self.state
    }

    fn get_memory(&mut self) -> &Vec<Message>  {
        &self.memory
    }

}