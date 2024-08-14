use strum::FromRepr;

pub mod config;

#[derive(FromRepr, Debug, Clone)]
#[repr(u8)]
pub enum Location {
    EXTERNAL = 1,
    KRAKEN = 2,
    POLONIEX = 3,
    BITTREX = 4,
    BINANCE = 5,
    BITMEX = 6,
    COINBASE = 7,
    TOTAL = 8,
    BANKS = 9,
    BLOCKCHAIN = 10,
    COINBASEPRO = 11,
    GEMINI = 12,
    EQUITIES = 13,
    REALESTATE = 14,
    COMMODITIES = 15,
    CRYPTOCOM = 16,
    UNISWAP = 17,
    BITSTAMP = 18,
    BINANCEUS = 19,
    BITFINEX = 20,
    BITCOINDE = 21,
    ICONOMI = 22,
    KUCOIN = 23,
    BALANCER = 24,
    LOOPRING = 25,
    FTX = 26,
    NEXO = 27,
    BLOCKFI = 28,
    INDEPENDENTRESERVE = 29,
    GITCOIN = 30,
    SUSHISWAP = 31,
    SHAPESHIFT = 32,
    UPHOLD = 33,
    BITPANDA = 34,
    BISQ = 35,
    FTXUS = 36,
    OKX = 37,
    ETHEREUM = 38,
    OPTIMISM = 39,
    POLYGONPOS = 40,
    ARBITRUMONE = 41,
    BASE = 42,
    GNOSIS = 43,
    WOO = 44,
    BYBIT = 45,
    SCROLL = 46,
    ZKSYNCLITE = 47,
    HTX = 48
}

impl From<String> for Location {
    fn from(value: String) -> Self {
        Location::from_repr(Vec::from(value)[0] as u8 - 64).unwrap()
    }
}

impl Into<String> for Location {
    fn into(self) -> String {
        format!("{:?}", (self as u8) as char)
    }
}