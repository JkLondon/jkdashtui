use color_eyre::Result;
use rand::Rng;
use std::{sync::mpsc, thread, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });
    let tx_to_btc_price_events = event_tx.clone();
    thread::spawn(move || {
        run_btc_price_thread(tx_to_btc_price_events);
    });
    let tx_to_weather_events = event_tx.clone();
    thread::spawn(move || {
        run_weather_thread(tx_to_weather_events);
    });
    let result = app.run(terminal, event_rx);
    ratatui::restore();
    result
}

pub enum Event {
    Input(crossterm::event::KeyEvent),
    BTCPrice(String),
    Weather(String),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn get_btc_price() -> Option<String> {
    let num = rand::rng().random_range(0..100);
    let res = 100000 + 1123 * num;
    Some(res.to_string())
}

fn get_weather() -> Option<String> {
    let weather_variants = ["Sunny", "Rain", "Clouds", "Snow", "Storm"];
    let num = rand::rng().random_range(0..5);
    Some(weather_variants[num].to_string())
}

fn run_btc_price_thread(tx: mpsc::Sender<Event>) {
    loop {
        thread::sleep(Duration::from_millis(200));
        let btc_price = get_btc_price();
        tx.send(Event::BTCPrice(btc_price.unwrap())).unwrap();
    }
}

fn run_weather_thread(tx: mpsc::Sender<Event>) {
    loop {
        thread::sleep(Duration::from_millis(1000));
        let weather = get_weather();
        tx.send(Event::Weather(weather.unwrap())).unwrap();
    }
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
    pub fn run(mut self, mut terminal: DefaultTerminal, rx: mpsc::Receiver<Event>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events(&rx)?;
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self, rx: &mpsc::Receiver<Event>) -> Result<()> {
        match rx.recv().unwrap() {
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
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
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
            "Weather is: ".into(),
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
