use std::sync::{Mutex, Arc};

use crate::{OrderbookSnapshot, level::Level, aggregator::Aggregator};
use serde_json::Value;
use url::Url;
use tungstenite::connect;

fn deserialize(raw: &str) -> Result<OrderbookSnapshot<10>, ()> {
    let deserialized: Value = serde_json::from_str(raw).unwrap();
    let exchange = ['B', 'I', 'N', 'A', 'N', 'C', 'E', ' ', ' ', ' '];
    let bids = &deserialized["bids"];
    let asks = &deserialized["asks"];

    Ok(OrderbookSnapshot { 
        bids: [
            Level {
                exchange,
                price: bids[0][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[0][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[1][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[1][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[2][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[2][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[3][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[3][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[4][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[4][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[5][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[5][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[6][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[6][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[7][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[7][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[8][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[8][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: bids[9][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: bids[9][1].as_str().unwrap().parse::<f64>().unwrap()
            }
        ], 
        asks: [
            Level {
                exchange,
                price: asks[0][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[0][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[1][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[1][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[2][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[2][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[3][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[3][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[4][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[4][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[5][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[5][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[6][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[6][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[7][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[7][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[8][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[8][1].as_str().unwrap().parse::<f64>().unwrap()
            },
            Level {
                exchange,
                price: asks[9][0].as_str().unwrap().parse::<f64>().unwrap(),
                amount: asks[9][1].as_str().unwrap().parse::<f64>().unwrap()
            }
        ] 
    })
}

pub fn run_stream(aggregator_arc: Arc<Mutex<Aggregator<10>>>) {
    let (mut socket, _) = connect(
        Url::parse("wss://stream.binance.com:9443/ws/ethbtc@depth10@100ms").unwrap()
    ).expect("Unable to connect to Binance Exchange");

    loop {
        let msg = socket.read_message().expect("Unable to read from message from Binance websocket stream");
        let content = msg.into_text().expect("Unable to read from message from Binance websocket stream");
        let deserialization = deserialize(&content);

        if let Ok(snapshot) = deserialization {
            
            if let Ok(mut aggregator) = aggregator_arc.lock() {
                let summary = aggregator.next("Binance", snapshot);
                println!("{}", summary.spread);
            }
        }
    }
}