# Orderbook Aggregator
This project was built as part of a coding challenge in an interview process. 
A server will stream the orderbook snapshots from two different exchanges, aggregate them
and publish them over gRPC channel. The client will render the aggregation in the console:

![client-sample](https://github.com/int_0x81/keyrock_challenge/raw/main/client_sample.png "Client Sample")

## Start the server

```
cd src/server
cargo run --release
```

## Start the client

```
cd src/client
cargo run --release
```