use super::{Agent, AgentError};

pub trait Agents {
    fn get_one_by_id(&mut self, id: String) ->  Result<Agent, AgentError>;
}
