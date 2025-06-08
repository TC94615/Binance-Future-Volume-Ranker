This Rust application retrieves active trading pairs from the Binance Futures market, sorts them in descending order of their 24-hour trading volume (in the quote asset, e.g., USDT), and allows filtering by a minimum specified quote volume.

To run: cargo run -- --min-quote-volume <TARGET_VOLUME> (e.g., 1000000 for pairs with >1M USDT volume)
