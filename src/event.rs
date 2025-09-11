pub enum Event {
    Input(crossterm::event::KeyEvent),
    BTCPrice(String),
    Weather(String),
}