mod layout;
mod widgets;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};

use crate::app::App;

pub fn render(app: &App, f: &mut Frame) {
    let area = f.area();
    let buf = f.buffer_mut();
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
        app.state.btc_price.clone().yellow(),
    ])]);

    let weather_text = Text::from(vec![Line::from(vec![
        "Weather: ".into(),
        app.state.weather.clone().yellow(),
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
