#[derive(Debug, Clone)]
pub struct Agent {
    pub options: AgentOptions,
}

impl Agent {
    pub fn new(options: AgentOptions) -> Self {
        Self { options }
    }
}
