use tokio::sync::mpsc::UnboundedSender;
use crate::{event::Event};
use tokio::task;

pub fn spawn_all(tx: UnboundedSender<Event>, client: reqwest::Client) {
    let tx_in = tx.clone();
    task::spawn(async move {super::tasks::input::run(tx_in).await});

    let tx_btc = tx.clone();
    let client_btc = client.clone();
    task::spawn(async move {super::tasks::btc::run(tx_btc, &client_btc).await});

    let tx_weather = tx.clone();
    let client_weather = client.clone();
    task::spawn(async move {super::tasks::weather::run(tx_weather, &client_weather).await});
}

pub mod input;
pub mod btc;
pub mod weather;