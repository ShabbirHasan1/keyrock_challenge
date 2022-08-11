use std::sync::Arc;

use crate::{aggregator::Aggregator, OrderbookSnapshot};
use serde_json::Value;
use tokio::sync::Mutex;
use tungstenite::connect;
use url::Url;

use keyrock_challenge_proto::orderbook::Level;

fn deserialize(raw: &str) -> Result<OrderbookSnapshot<10>, ()> {
    let deserialization = serde_json::from_str(raw);

    let deserialized: Value = match deserialization {
        Ok(des) => des,
        Err(_) => return Err(()),
    };

    let exchange = "Binance";
    let bids = &deserialized["bids"];
    let asks = &deserialized["asks"];

    if bids.is_null() || asks.is_null() {
        return Err(());
    }

    Ok(OrderbookSnapshot {
        bids: [
            Level {
                exchange: exchange.to_string(),
                price: bids[0][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[0][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[1][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[1][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[2][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[2][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[3][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[3][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[4][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[4][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[5][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[5][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[6][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[6][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[7][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[7][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[8][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[8][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: bids[9][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[9][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
        ],
        asks: [
            Level {
                exchange: exchange.to_string(),
                price: asks[0][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[0][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[1][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[1][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[2][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[2][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[3][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[3][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[4][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[4][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[5][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[5][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[6][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[6][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[7][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[7][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[8][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[8][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
            Level {
                exchange: exchange.to_string(),
                price: asks[9][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[9][1].as_str().unwrap().parse::<f64>().unwrap(),
            },
        ],
    })
}

pub async fn run_stream(source_id: usize, aggregator_arc: Arc<Mutex<Aggregator>>) {
    let (mut socket, _) =
        connect(Url::parse("wss://stream.binance.com:9443/ws/ethbtc@depth10@100ms").unwrap())
            .expect("Unable to connect to Binance Exchange");

    loop {
        let msg = socket
            .read_message()
            .expect("Unable to read from message from Binance websocket stream");
        let content = msg
            .into_text()
            .expect("Unable to read from message from Binance websocket stream");
        let deserialization = deserialize(&content);

        if let Ok(snapshot) = deserialization {
            aggregator_arc.lock().await.process(source_id, snapshot);
        }
    }
}
