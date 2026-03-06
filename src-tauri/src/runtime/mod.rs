use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct RuntimeEvent {
    pub correlation_id: String,
    pub kind: String,
    pub payload_json: Option<String>,
}

pub struct RuntimeSkeleton {
    pub sender: mpsc::Sender<RuntimeEvent>,
    pub receiver: mpsc::Receiver<RuntimeEvent>,
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
