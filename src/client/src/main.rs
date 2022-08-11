mod console_renderer;

use keyrock_challenge_proto::orderbook::{orderbook_aggregator_client, Empty};
use tokio_stream::StreamExt;

const SERVER_URL: &str = "http://[::1]:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        orderbook_aggregator_client::OrderbookAggregatorClient::connect(SERVER_URL).await?;

    let mut stream = client.book_summary(Empty {}).await?.into_inner();

    while let Some(summary) = stream.next().await {
        console_renderer::render(summary.unwrap());
    }
    Ok(())
}
