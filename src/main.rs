use jkdashtui::{app::App, tasks, services::http, event::Event, Result};
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let (tx, rx) = unbounded_channel::<Event>();

    let client = http::new_http_client();

    tasks::spawn_all(tx.clone(), client.clone());

    let app = App::new();

    app.run(terminal, rx).await?;
    ratatui::restore();
    Ok(())
}