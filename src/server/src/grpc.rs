use crate::spmc::Spmc;
use keyrock_challenge_proto::orderbook::{
    orderbook_aggregator_server::OrderbookAggregator, Empty, Summary,
};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Response, Status};

const SPMC_BUFFER_SIZE: usize = 64;
const GRPC_BUFFER_SIZE: usize = 64;

#[derive(Debug)]
pub struct OrderbookAggregatorServer {
    spmc: Arc<Mutex<Spmc>>,
}

impl OrderbookAggregatorServer {
    pub fn new(spmc: Arc<Mutex<Spmc>>) -> OrderbookAggregatorServer {
        OrderbookAggregatorServer { spmc }
    }
}

type BookSummaryResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookAggregatorServer {
    type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<Summary, Status>> + Send>>;

    async fn book_summary(
        &self,
        _: tonic::Request<Empty>,
    ) -> BookSummaryResult<Self::BookSummaryStream> {
        let mut spmc = self.spmc.lock().await;
        let mut rx = spmc.create_receiver(SPMC_BUFFER_SIZE);
        let (stream_tx, stream_rx) = mpsc::channel(GRPC_BUFFER_SIZE);
        tokio::spawn(async move {
            loop {
                if let Some(new_summary) = rx.recv().await {
                    match stream_tx.send(Result::<_, Status>::Ok(new_summary)).await {
                        Ok(_) => {}
                        Err(_item) => {
                            drop(rx);
                            break;
                        }
                    }
                }
            }
        });

        let output_stream = ReceiverStream::new(stream_rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::BookSummaryStream
        ))
    }
}
