#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub counter: u8,
    pub weather: String,
    pub btc_price: String,
}