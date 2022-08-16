use keyrock_challenge_proto::orderbook::Summary;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct Spmc {
    senders: Vec<Sender<Summary>>,
}

impl Spmc {
    pub fn new() -> Self {
        Spmc {
            senders: Vec::<Sender<Summary>>::new(),
        }
    }

    pub async fn broadcast(&mut self, summary: Summary) {
        let mut index: usize = 0;

        loop {
            if index >= self.senders.len() {
                break;
            }
            let sender = &self.senders[index];
            let result = sender.send(summary.clone()).await;
            match result {
                Ok(_) => {
                    index += 1;
                }
                Err(_) => {
                    let _ = &self.senders.remove(index);
                }
            }
        }
    }

    pub fn create_receiver(&mut self, buffer: usize) -> Receiver<Summary> {
        let (tx, rx) = mpsc::channel(buffer);
        self.senders.push(tx);
        rx
    }
}
