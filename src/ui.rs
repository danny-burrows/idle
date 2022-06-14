use crate::app::Idle;
use tui::{
    backend::{Backend},
    widgets::{Block, List, ListItem, Borders, LineGauge, Sparkline, Paragraph},
    layout::{Layout, Alignment, Constraint, Direction},
    style::{Style, Modifier, Color},
    symbols,
    Frame
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Idle) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25)
            ].as_ref()
        )
        .split(f.size());
    
    let block = Block::default()
        .title("Sub Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let items: Vec<ListItem> = app.incrementors.list.iter().map(|f| ListItem::new(f.as_ref())).collect();
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunks[1], &mut app.incrementors.state);


    // Left Chunk
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunks[0]);

    let sparkline_width = main_chunks[0].width.into();
    app.sparkline_max_length = sparkline_width;


    if app.sparkline_data.len() > sparkline_width {
        
        let kill_old = app.sparkline_data.len() - sparkline_width;

        app.sparkline_data.drain(0..kill_old);
    }

    let sparkline = Sparkline::default()
        .block(Block::default().title("Sparkline:"))
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline_data)
        .bar_set(symbols::bar::NINE_LEVELS);
    f.render_widget(sparkline, main_chunks[0]);

    let line_gauge = LineGauge::default()
        .block(Block::default().title("LineGauge:"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .line_set(symbols::line::NORMAL)
        .ratio(0.69);
    f.render_widget(line_gauge, main_chunks[1]);

    let block = Block::default()
    .title("Main Block")
    .borders(Borders::ALL);

    let paragraph = Paragraph::new(format!("Clicks {}", app.total_clicks))
        .style(Style::default().fg(Color::Red))
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, main_chunks[2]);
}
