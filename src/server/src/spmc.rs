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

    pub async fn broadcast(&self, summary: Summary) {
        for sender in &self.senders {
            let result = sender.send(summary.clone()).await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    println!("Unable to broadcast: {}", err)
                } //todo handle client dc
            }
        }
    }

    pub fn create_receiver(&mut self, buffer: usize) -> Receiver<Summary> {
        let (tx, rx) = mpsc::channel(buffer);
        self.senders.push(tx);
        rx
    }
}
