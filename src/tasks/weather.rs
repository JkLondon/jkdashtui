use tokio::time::{self, Duration};
use tokio::sync::mpsc::UnboundedSender;
use crate::{event::Event, services::weather, Result};

pub async fn run(tx: UnboundedSender<Event>, client: &reqwest::Client) -> Result<()> {
    let mut tick = time::interval(Duration::from_millis(5000));
    loop {
        let weather = weather::get_weather(client).await?;
        if tx.send(Event::Weather(weather)).is_err() {
            break;
        }
        tick.tick().await;
    }
    Ok(())
}