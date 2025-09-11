mod state;
pub mod actions;

use crate::{event::Event, ui, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct App {
    pub state: state::AppState,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { state: state::AppState::default(), running: true }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal, mut rx: UnboundedReceiver<Event>) -> Result<()> {
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

    fn handle_event(&mut self, ev: Event) -> Result<()> {
        use Event::*;
        match ev {
            Input(key)        => self.on_key(key),
            BTCPrice(p)       => self.state.btc_price = p,
            Weather(w)        => self.state.weather = w,
        };
        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.running = false,
            _ => {}
        }
    }

    fn draw(&self, f: &mut Frame) { ui::render(self, f)}
}