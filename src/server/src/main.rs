mod aggregator;
mod binance_spot;
mod bitstamp_spot;
mod grpc;
mod orderbook_snapshot;

use aggregator::Aggregator;
use grpc::OrderbookAggregatorServer;
use orderbook_snapshot::OrderbookSnapshot;

use keyrock_challenge_proto::orderbook::{self, Summary};
use tokio::sync::watch::{self, Receiver, Sender};
use tonic::transport::Server;

use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
    thread,
};

async fn start_exchange_streams(
    aggregator: Arc<Mutex<Aggregator>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let aggregator_02 = aggregator.clone();

    let binance_stream = thread::spawn(move || {
        binance_spot::run_stream(0, aggregator);
    });

    let bitstamp_stream = thread::spawn(move || {
        bitstamp_spot::run_stream(1, aggregator_02);
    });

    binance_stream.join().unwrap();
    bitstamp_stream.join().unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (Sender<Option<Summary>>, Receiver<Option<Summary>>) = watch::channel(None);
    let aggregator: Aggregator = Aggregator::new(tx);
    let aggregator = Mutex::new(aggregator);
    let aggregator = Arc::new(aggregator);

    let exchange_stream = start_exchange_streams(aggregator.clone());

    let server = OrderbookAggregatorServer::new(rx);
    Server::builder()
        .add_service(orderbook::orderbook_aggregator_server::OrderbookAggregatorServer::new(server))
        .serve("[::1]:8080".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    exchange_stream.await
}
