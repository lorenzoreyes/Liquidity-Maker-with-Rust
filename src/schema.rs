syntax = "proto3";

package orderbook;

service OrderbookAggregator {
    rpc BookSummary(Empty) returns (stream Summary);
}

message empty {}

message Summary {
    double spread = 1;
    repeated Level bids = 2;
    repeated Level asks = 3;
}

message Level {
    string exchange = 1;
    double price = 2;
    double amount = 3;
}
