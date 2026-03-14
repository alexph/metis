use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct RuntimeEvent {
    pub correlation_id: String,
    pub kind: String,
    pub payload_json: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RuntimeCommand {
    ProcessTurn {
        channel_id: String,
        trigger_history_id: String,
        correlation_id: String,
    },
}

pub struct RuntimeSkeleton {
    pub sender: mpsc::Sender<RuntimeCommand>,
    pub receiver: mpsc::Receiver<RuntimeCommand>,
}

impl RuntimeSkeleton {
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer);
        Self { sender, receiver }
    }

    pub async fn run_once(&mut self) {
        let _ = self.receiver.recv().await;
    }
}

pub async fn run_forever(mut receiver: mpsc::Receiver<RuntimeCommand>) {
    while let Some(command) = receiver.recv().await {
        tracing::debug!(?command, "runtime command received");
    }
}
