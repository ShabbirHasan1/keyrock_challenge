use crate::{OrderbookSnapshot, aggregator::Aggregator, level::Level};
use std::sync::{Mutex, Arc};
use serde_json::Value;
use url::Url;
use tungstenite::{connect, Message};

fn deserialize(raw: &str) -> Result<OrderbookSnapshot<10>, ()> {
    let deserialization = serde_json::from_str(raw);
    let deserialized: Value;

    match deserialization {
      Ok(des) => deserialized = des,
      Err(_) => return Err(())
    }

    let exchange = ['B', 'I', 'T', 'S', 'T', 'A', 'M', 'P', ' ', ' '];
    let data = &deserialized["data"];
    let bids = &data["bids"];
    let asks = &deserialized["data"]["asks"];

    if bids.is_null() || asks.is_null() {
      return Err(());
    }

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
        Url::parse("wss://ws.bitstamp.net/").unwrap()
    ).expect("Unable to connect to Bitstamp Exchange");

    socket.write_message(Message::Text(r#"
        {
          "event": "bts:subscribe",
          "data": {
            "channel": "detail_order_book_ethbtc"
          }
        }
    "#.into())).expect("Unable to write message to Bitstamp websocket stream");

    loop {
        let msg = socket.read_message().expect("Unable to read from message from Bitstamp websocket stream");
        let content = msg.into_text().expect("Unable to read from message from Bitstamp websocket stream");
        let deserialization = deserialize(&content);

        match deserialization {
          Ok(snapshot) => {
            if let Ok(mut aggregator) = aggregator_arc.lock() {
              let summary = aggregator.next("Bitstamp", snapshot);
              println!("{}", summary.spread);
            }
          },
          Err(_) => {}
        }
    }
}