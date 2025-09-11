use reqwest::Client;
use serde::Deserialize;

use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use tokio::task;
use tokio::time::{self, Duration};
use color_eyre::Result;

use crossterm::event::{read, Event as CtEvent, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Deserialize)]
struct BinancePrice {
    price: String
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new();
    let http_client = reqwest::Client::new();
    let btc_clone = http_client.clone();
    let weather_clone = http_client.clone();
    let (event_tx, event_rx) = unbounded_channel::<Event>();

    tokio::spawn({ let tx = event_tx.clone(); async move { input_events_task(tx).await}});
    tokio::spawn({ let tx = event_tx.clone(); async move { run_btc_price_task(tx, &btc_clone).await}});
    tokio::spawn({ let tx = event_tx.clone(); async move { run_weather_task(tx, &weather_clone).await}});

    app.run(terminal, event_rx).await?;
    ratatui::restore();
    Ok(())
}

pub enum Event {
    Input(crossterm::event::KeyEvent),
    BTCPrice(String),
    Weather(String),
}

async fn input_events_task(tx: UnboundedSender<Event>) {
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

async fn get_btc_price(client: &Client) -> color_eyre::Result<String> {
    let bp: BinancePrice = client
    .get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
    .send()
    .await?
    .json()
    .await?;

    let price = bp.price;
    Ok(price)
}

async fn get_weather(client: &Client) -> color_eyre::Result<String> {
    let resp = client
    .get("https://wttr.in/Gijon?format=4")
    .send()
    .await?
    .text()
    .await?;

    Ok(resp)
}

async fn run_btc_price_task(tx: UnboundedSender<Event>, client: &Client) -> color_eyre::Result<()> {
    let mut tick = time::interval(Duration::from_millis(500));
    loop {
        tick.tick().await;
        let btc_price = get_btc_price(client).await?;
        if tx.send(Event::BTCPrice(btc_price)).is_err() {
            break;
        }
    }
    Ok(())
}

async fn run_weather_task(tx: UnboundedSender<Event>, client: &Client) -> color_eyre::Result<()> {
    let mut tick = time::interval(Duration::from_millis(5000));
    loop {
        tick.tick().await;

        let weather = get_weather(client).await?;
        if tx.send(Event::Weather(weather)).is_err() {
            break;
        }
    }
    Ok(())
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    btc_price: String,
    weather: String,
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal, mut rx: UnboundedReceiver<Event>) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            if let Some(event) = rx.recv().await {
                self.handle_event(event)?;
                terminal.draw(|frame| self.draw(frame))?;
            } else {
                break;
            }
            
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            // Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            // Event::Mouse(_) => {}
            // Event::Resize(_, _) => {}
            Event::Input(key_event) => self.on_key_event(key_event),
            Event::BTCPrice(btc_price) => self.btc_price = btc_price,
            Event::Weather(weather) => self.weather = weather,
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Left) => self.decrement_counter(),
            (_, KeyCode::Right) => self.increment_counter(),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" JkDashTUI ".bold());
        let instructions = Line::from(vec![
            "Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let bg_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(bg_layout[1]);

        Block::bordered()
            .title(title.centered())
            .border_set(border::EMPTY).render(bg_layout[0], buf);
        Block::bordered()
            .title_bottom(instructions.centered())
            .border_set(border::EMPTY).render(bg_layout[2], buf);
        let block_btc_price = Block::bordered()
            .border_set(border::THICK);
        let block_weather = Block::bordered()
            .border_set(border::THICK);
        let btc_price_text = Text::from(vec![Line::from(vec![
            "BTC Price: ".into(),
            self.btc_price.clone().yellow(),
        ])]);

        let weather_text = Text::from(vec![Line::from(vec![
            "Weather: ".into(),
            self.weather.clone().yellow(),
        ])]);
        
        Paragraph::new(btc_price_text)
            .centered()
            .block(block_btc_price)
            .render(content_layout[0], buf);
        Paragraph::new(weather_text)
            .centered()
            .block(block_weather)
            .render(content_layout[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
