use crate::app::Idle;
use tui::{
    backend::{Backend},
    widgets::{Block, List, ListItem, Borders, LineGauge, Sparkline, Paragraph, Chart, Axis, Dataset, GraphType},
    layout::{Layout, Rect, Alignment, Constraint, Direction},
    style::{Style, Modifier, Color},
    text::Span,
    symbols,
    Frame
};

fn style_title<'a>(title_text: &'a str) -> Span<'a> { 
    Span::styled(
        title_text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    )
}

fn draw_shop_chunk<B: Backend>(f: &mut Frame<B>, app: &mut Idle, chunk_rect: Rect) {
    let items: Vec<ListItem> = app.incrementors.list.iter().map(|f| ListItem::new(f.as_ref())).collect();
    let list = List::new(items)
        .block(Block::default().title(style_title("Shop")).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk_rect, &mut app.incrementors.state);
}

fn draw_sparkline<B: Backend>(f: &mut Frame<B>, app: &mut Idle, rect: Rect) {
    let sparkline_width = rect.width.into();
    app.sparkline_max_length = sparkline_width;


    if app.sparkline_data.len() > sparkline_width {
        
        let kill_old = app.sparkline_data.len() - sparkline_width;

        app.sparkline_data.drain(0..kill_old);
    }

    let clicks_this_tick = if app.sparkline_data.len() > 0 {
        app.sparkline_data[app.sparkline_data.len() - 1]
    } else {
        0
    };

    let avg_clicks_per_tick = {
        let sum: u64 = app.sparkline_data.iter().sum();
        sum as f64 / (app.sparkline_data.len() as f64)
    };

    let titlett = format!("Clicks / Tick (This Tick: {}) (Avg: {:.2})", clicks_this_tick, avg_clicks_per_tick);

    let sparkline = Sparkline::default()
        .block(Block::default().title(style_title(titlett.as_str())))
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline_data)
        .bar_set(symbols::bar::NINE_LEVELS);
    f.render_widget(sparkline, rect);
}

fn draw_graph<B: Backend>(f: &mut Frame<B>, app: &mut Idle, rect: Rect) {
    let mut use_data = vec![];
    let mut i = 0.0;
    for it in &app.graph_data {
        use_data.push((i, *it as f64));
        i += 1.0;
    }

    let mut max_y = 10.0;
    if let Some(t) = app.graph_data.iter().max() {
        max_y = *t as f64 * 1.15;
    }

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&use_data)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(style_title("Total Clicks / Ticks"))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, use_data.len() as f64])
        )
        .y_axis(
            Axis::default()
                .title("Total Clicks")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_y])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled(format!("{:.0}", max_y), Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, rect);
}

fn draw_loading_bar<B: Backend>(f: &mut Frame<B>, _app: &mut Idle, rect: Rect) {
    let line_gauge = LineGauge::default()
        .block(Block::default().title(style_title("LineGauge")))
        .gauge_style(Style::default().fg(Color::Magenta))
        .line_set(symbols::line::NORMAL)
        .ratio(0.20);
    f.render_widget(line_gauge, rect);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Idle) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25)
            ].as_ref()
        )
        .split(f.size());

    // Right Chunk
    draw_shop_chunk(f, app, chunks[1]);

    // Left Chunk
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Min(2),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(chunks[0]);

    draw_sparkline(f, app, main_chunks[0]);

    draw_graph(f, app, main_chunks[1]);

    draw_loading_bar(f, app, main_chunks[2]);

    let block = Block::default()
    .title(style_title("Main Block"))
    .borders(Borders::ALL);

    let paragraph = Paragraph::new(format!("Clicks {}", app.total_clicks))
        .style(Style::default().fg(Color::Gray))
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, main_chunks[3]);
}
