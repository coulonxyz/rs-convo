use std::fs;
use super::{Agent, Agents, AgentError};

struct FsAgents {
    path: String,
}

impl FsAgents {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn read_file(&self, file_name: &str) -> Option<String> {
        let path = format!("{}/{}.json", self.path, file_name);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Reversal Attack Attempted: {}", file_name);
                    None
                }
            },
            Err(_) => None,
        }
    }

}

impl Agents for FsAgents {
    fn get_one_by_id(&mut self, id: String) -> Result<Agent, AgentError> {
        match self.read_file(&id) {
            Some(raw_data) => {
                match serde_json::from_str(&raw_data) {
                    Ok(agent) => { Ok(agent) },
                    Err(_) => { Err(AgentError::BadFormat) }
                }
            }
            None => { return Err(AgentError::NotFound) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_agents_path() -> String { format!("{}/tests/fixtures/agents", env!("CARGO_MANIFEST_DIR")) }

    #[test]
    fn gets_agent() {
        let agent = FsAgents::new(get_test_agents_path())
            .get_one_by_id(String::from("agent_1"));
        assert!(agent.is_ok())
    }

    #[test]
    fn fails_to_get_agent_if_can_t_be_found() {
        let agent = FsAgents::new(get_test_agents_path())
            .get_one_by_id(String::from("imaginary_agent"));
        assert_matches!(
            agent,
            Err(AgentError::NotFound)
            );
    }

    #[test]
    fn fails_to_get_agent_if_its_format_is_not_good() {
        let agent = FsAgents::new(get_test_agents_path())
            .get_one_by_id(String::from("agent_bad_format"));
        assert_matches!(
            agent,
            Err(AgentError::BadFormat)
            );
    }
}

