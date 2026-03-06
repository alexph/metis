#[derive(Debug, Clone)]
pub struct AgentRequest {
    pub prompt: String,
    pub correlation_id: String,
}

pub trait AgentClient {
    fn respond(&self, request: AgentRequest) -> Result<String, &'static str>;
}

pub struct RigAgentStub;

impl AgentClient for RigAgentStub {
    fn respond(&self, _request: AgentRequest) -> Result<String, &'static str> {
        Ok("hello".to_string())
    }
}
