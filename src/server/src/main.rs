mod aggregator;
mod binance_spot;
mod bitstamp_spot;
mod grpc;
mod orderbook_snapshot;

use aggregator::Aggregator;
use grpc::OrderbookAggregatorServer;
use orderbook_snapshot::OrderbookSnapshot;

use keyrock_challenge_proto::orderbook::{self, Summary};
use tokio::sync::{
    watch::{self, Receiver, Sender},
    Mutex,
};
use tonic::transport::Server;

use std::{net::ToSocketAddrs, sync::Arc};

const SERVER_URL: &str = "[::1]:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (Sender<Option<Summary>>, Receiver<Option<Summary>>) = watch::channel(None);
    let aggregator: Aggregator = Aggregator::new(tx);
    let aggregator = Mutex::new(aggregator);
    let aggregator = Arc::new(aggregator);

    let agg_01 = aggregator.clone();
    let agg_02 = aggregator.clone();

    let binance_stream = tokio::spawn(async move { binance_spot::run_stream(0, agg_01).await });
    let bitstamp_stream = tokio::spawn(async move { bitstamp_spot::run_stream(1, agg_02).await });

    let server = OrderbookAggregatorServer::new(rx);
    Server::builder()
        .add_service(orderbook::orderbook_aggregator_server::OrderbookAggregatorServer::new(server))
        .serve(SERVER_URL.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    binance_stream.await?;
    bitstamp_stream.await?;
    Ok(())
}
