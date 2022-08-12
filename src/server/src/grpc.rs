use keyrock_challenge_proto::orderbook::{
    orderbook_aggregator_server::OrderbookAggregator, Empty, Summary,
};
use std::pin::Pin;
use tokio::sync::{mpsc, watch::Receiver};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Response, Status};

#[derive(Debug)]
pub struct OrderbookAggregatorServer {
    summary_receiver: Receiver<Option<Summary>>,
}

impl OrderbookAggregatorServer {
    pub fn new(summary_receiver: Receiver<Option<Summary>>) -> OrderbookAggregatorServer {
        OrderbookAggregatorServer { summary_receiver }
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
        let mut summary_receiver = self.summary_receiver.clone();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            loop {
                summary_receiver.changed().await.unwrap();
                let new_summary = summary_receiver.borrow().clone().unwrap();
                match tx.send(Result::<_, Status>::Ok(new_summary)).await {
                    Ok(_) => {}
                    Err(_item) => {}
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::BookSummaryStream
        ))
    }
}
