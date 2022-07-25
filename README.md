# This project ia my approach for the Keyrock-Challenge for Rust
*** Also this documents aims to record my walkthrough in this journey
*** As a developer I aim that my code speak for itself and on my behalf

### Disclaimer: The pythonWay is a python approach that I did in order
### to have an image of what were the steps needed to do the challenge.

The goals aimed for the software include:

 (1) Connection to two exchanges simmultaneously (Binance & Bitsmap)
// generate two streams to generate connection & merge them.

 (2) Pull the order book from a configurable currencies
// Set configuration as a variable in order the user can test it with different pairs

 (3) Combine the results.
// Data cleaning
 (4) From the result publish the spread, top ten bid & asks through gRPC server as a stream.


MENTAL NOTE: I am afraid yet excited to see what results I gain.

TO DO:
(1) Connection to Websockets - [x]
(2) Merge Connections - [ ]
(3) Pulls orderbooks bids & asks - [x]
(4) Merge and sort orderbooks - [ ]
(5) Obtain the spread (ask-bid/ask), top ten bids, top ten asks && publish it as a stream.- [ ]
