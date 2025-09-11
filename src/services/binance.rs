use crate::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct BinancePrice {
    price: String
}

pub async fn get_btc_price(client: &reqwest::Client) -> Result<String> {
    let bp: BinancePrice = client
    .get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
    .send()
    .await?
    .json()
    .await?;

    Ok(bp.price)
}