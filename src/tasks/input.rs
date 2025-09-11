use tokio::task;
use tokio::sync::mpsc::UnboundedSender;
use crossterm::event::{read, Event as CtEvent};
use crate::event::Event;

pub async fn run(tx: UnboundedSender<Event>) {
    loop {
        let ev = match task::spawn_blocking(|| read()).await {
            Ok(Ok(e)) => e,
            Ok(Err(_e)) => continue,
            Err(_join_err) => break,
        };

        if let CtEvent::Key(key) = ev {
            let _ = tx.send(Event::Input(key));
        }
    }
}