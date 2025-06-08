use clap::{Arg, Command};
use reqwest::blocking::get;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct Ticker {
    symbol: String,
    #[serde(deserialize_with = "deserialize_quote_volume", rename = "quoteVolume")]
    quote_volume: f64,
}

#[derive(Deserialize)]
struct ExchangeInfo {
    symbols: Vec<SymbolInfo>,
}

#[derive(Deserialize)]
struct SymbolInfo {
    symbol: String,
    status: String,
}

fn deserialize_quote_volume<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

fn setup_matches() -> clap::ArgMatches {
    let matches = Command::new("Binance Ticker Filter")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Filters Binance ticker data by quote volume")
        .arg(
            Arg::new("min-quote-volume")
                .short('m')
                .long("min-quote-volume")
                .value_name("VOLUME")
                .help("Sets the minimum quote volume")
                .required(true),
        )
        .get_matches();
    matches
}

fn get_min_quote_volume_from(matches: clap::ArgMatches) -> f64 {
    let min_quote_volume: f64 = matches
        .get_one::<String>("min-quote-volume")
        .expect("Required argument not found")
        .parse()
        .expect("Invalid number for min_quote_volume");
    min_quote_volume
}

fn fetch_trading_symbols() -> Vec<String> {
    // 獲取交易對的狀態
    let exchange_info_url = "https://fapi.binance.com/fapi/v1/exchangeInfo";
    let exchange_info_response: ExchangeInfo = get(exchange_info_url)
        .expect("Failed to fetch exchange info")
        .json()
        .expect("Failed to parse exchange info");

    // 提取所有狀態為 "TRADING" 的交易對
    exchange_info_response
        .symbols
        .iter()
        .filter_map(|symbol_info| {
            if symbol_info.status == "TRADING" {
                Some(symbol_info.symbol.clone())
            } else {
                None
            }
        })
        .collect()
}

fn fetch_tickers() -> Vec<Ticker> {
    let url = "https://fapi.binance.com/fapi/v1/ticker/24hr";
    let response: Vec<Ticker> = get(url)
        .expect("Failed to fetch data")
        .json()
        .expect("Failed to parse JSON");
    response
}

fn filter_tickers(tickers: Vec<Ticker>, trading_symbols: Vec<String>, min_quote_volume: f64) -> Vec<Ticker> {
    tickers
        .into_iter() // 使用 into_iter() 來消耗 tickers 向量
        .filter(|ticker| trading_symbols.contains(&ticker.symbol))
        .filter(|ticker| ticker.quote_volume >= min_quote_volume)
        .collect() // 收集到 Vec<Ticker>
}

fn sort_tickers(tickers: Vec<Ticker>) -> Vec<Ticker> {
    let mut sorted_tickers = tickers; // 創建一個可變的副本
    sorted_tickers.sort_by(|a, b| b.quote_volume.partial_cmp(&a.quote_volume).unwrap());
    sorted_tickers // 返回排序後的向量
}

fn get_symbols(filtered_tickers: Vec<Ticker>) -> Vec<String> {
    let symbols: Vec<String> = filtered_tickers
        .iter()
        .map(|ticker| format!("{}.P", ticker.symbol))
        .collect();
    symbols
}

fn main() {
    // 定義命令行參數
    let matches = setup_matches();
    // 讀取參數值並轉換為 f64
    let min_quote_volume = get_min_quote_volume_from(matches);
    // 獲取所有狀態為 "TRADING" 的交易對
    let trading_symbols = fetch_trading_symbols();
    // 獲取 Binance API 數據
    let tickers = fetch_tickers();
    // 過濾交易對
    let filtered_tickers = filter_tickers(tickers, trading_symbols, min_quote_volume);
    // 根據 quote_volume 進行排序
    let sorted_tickers = sort_tickers(filtered_tickers);
    // 輸出結果
    let symbols = get_symbols(sorted_tickers);
    println!("{}", symbols.join(", "));
}