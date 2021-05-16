use super::Agents;
use std::fs;
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
pub struct Agent {
    id: String,
    name: String,
}

#[derive(Debug)]
pub enum AgentError {
    NotFound,
    BadFormat,
}

impl Agent {
    fn get_by_id(mut agents: impl Agents, id: String) -> Result<Agent, AgentError> {
        agents.get_one_by_id(id)
    }
}
