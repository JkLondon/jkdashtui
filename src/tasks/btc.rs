use tokio::time::{self, Duration};
use tokio::sync::mpsc::UnboundedSender;
use crate::{event::Event, services::binance, Result};

pub async fn run(tx: UnboundedSender<Event>, client: &reqwest::Client) -> Result<()> {
    let mut tick = time::interval(Duration::from_millis(500));
    loop {
        let btc_price = binance::get_btc_price(client).await?;
        if tx.send(Event::BTCPrice(btc_price)).is_err() {
            break;
        }
        tick.tick().await;
    }
    Ok(())
}