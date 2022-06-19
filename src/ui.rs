use crate::{app::Idle, shop::{Shop, ShopItem}};
use tui::{
    backend::Backend,
    widgets::{Block, List, ListItem, Borders, LineGauge, Sparkline, Paragraph, Chart, Axis, Dataset, GraphType},
    layout::{Layout, Rect, Constraint, Direction},
    style::{Style, Modifier, Color},
    text::{Spans, Span},
    symbols,
    Frame
};

fn style_title(title_text: &str) -> Span { 
    Span::styled(
        title_text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    )
}

fn draw_stats<B: Backend>(f: &mut Frame<B>, app: &Idle, chunk_rect: Rect) {

    // Draw stats border.
    let border_block = Block::default()
        .title(style_title(" Stats "))
        .borders(Borders::ALL);
    f.render_widget(border_block, chunk_rect);


    // Get the required number of constraints for all incrementors.
    let mut constraints: Vec<Constraint> = app.incrementors.list.iter().filter(|i| i.unlocked).map(|_f| {Constraint::Length(3)}).collect();

    // Extra constraint for stats.
    constraints.push(Constraint::Percentage(20));

    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(constraints)
        .split(chunk_rect);

    // Draw all incrementor gauges.
    let mut chnk = 0;
    for incrementor in app.incrementors.list.iter() {

        if !incrementor.unlocked {continue;}

        let r = incrementor.clicks / incrementor.max_clicks;
        
        let line_gauge = LineGauge::default()
            .block(Block::default().title(format!("{} +{:.2} (Total Earned: {:.2})", incrementor.name, incrementor.max_clicks, incrementor.total_earned)))
            .gauge_style(Style::default().fg(incrementor.colour))
            .line_set(symbols::line::NORMAL)
            .ratio(
                if r.is_nan() {0.0} else {r}
            );

        f.render_widget(line_gauge, stats_chunks[chnk]);
        chnk += 1;
    }

    // Draw stats block.
    let block = Block::default()
        .borders(Borders::TOP);
    
    let paragraph = Paragraph::new(
        format!(
            "\nClicks: {:.2} (+ {:.2})\n\nAll Time Clicks: {:.2}",
            app.total_clicks,
            0.0,
            app.all_time_total_clicks
        ))
        .block(block);
    f.render_widget(paragraph, stats_chunks[chnk]);
}

fn draw_shop<B: Backend>(f: &mut Frame<B>, app: &Idle, shop: &mut Shop, chunk_rect: Rect) {
    let items: Vec<ListItem> = shop.items.iter().map(|item| {

        match item {
            ShopItem::IncrementorPurchase{ text, price, colour, incrementor_index:_} => {
                ListItem::new(
                    Spans::from(vec![
                        Span::styled("• ", Style::default().fg(*colour)),
                        Span::raw(format!("{:<23} {:.2}", text, price)),
                    ])
                ).style(
                    // Style item green if it can be purchased.
                    if *price <= app.total_clicks {
                        Style::default().fg(Color::LightGreen)
                    } else {
                        Style::default()
                    }
                )
            }
            ShopItem::IncrementorUpgrade{ text, price, colour, incrementor_index:_} => {
                ListItem::new(
                    Spans::from(vec![
                        Span::styled("▲ ", Style::default().fg(*colour)),
                        Span::raw(format!("{:<23} {:.2}", text, price)),
                    ])
                ).style(
                    // Style item green if it can be purchased.
                    if *price <= app.total_clicks {
                        Style::default().fg(Color::LightGreen)
                    } else {
                        Style::default()
                    }
                )
            }
        }        
    }).collect();

    let list = List::new(items)
        .block(Block::default().title(style_title(" Shop ")).borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk_rect, &mut shop.state);
}

fn draw_sparkline<B: Backend>(f: &mut Frame<B>, app: &mut Idle, rect: Rect) {
    let sparkline_width = rect.width.into();
    app.sparkline_max_length = sparkline_width;

    // Drain overflowing data
    if app.sparkline_data.len() > sparkline_width {        
        app.sparkline_data.drain(0..(app.sparkline_data.len() - sparkline_width));
    }

    let clicks_this_tick = match app.sparkline_data.last().copied() {
        Some(i) => i, 
        _ => 0
    };

    let avg_clicks_per_tick = {
        let sum: u64 = app.sparkline_data.iter().sum();
        sum as f64 / (app.sparkline_data.len() as f64)
    };

    let titlett = format!(" Clicks / Tick (This Tick: {}) (Avg: {:.2}) ", clicks_this_tick, avg_clicks_per_tick);

    let sparkline = Sparkline::default()
        .block(Block::default().title(style_title(titlett.as_str())).borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline_data)
        .bar_set(symbols::bar::NINE_LEVELS);
    f.render_widget(sparkline, rect);
}

fn draw_graph<B: Backend>(f: &mut Frame<B>, app: &Idle, rect: Rect) {

    // Enumerate graph (y) data with its index (x).
    let use_data: Vec<(f64, f64)> = app.graph_data.iter().enumerate().map(|(i, it)| (i as f64, *it)).collect();

    // Calculate max y value for graph.
    let mut max_y = 10.0;
    if let Some(t) = app.graph_data.clone().into_iter().reduce(f64::max) {
        if t > 10.0 {
            max_y = t * 1.15;
        }
    }

    let datasets = vec![Dataset::default()
        .name("Current Clicks")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&use_data)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(style_title(" Total Clicks "))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, use_data.len() as f64])
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, max_y])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled(format!("{:.0}", max_y), Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, rect);
}

fn draw_chat<B: Backend>(f: &mut Frame<B>, _app: &Idle, rect: Rect) {
    let block = Block::default()
    .title(style_title(" Chat "))
    .borders(Borders::ALL);

    let paragraph = Paragraph::new(" Director: Welcome to idle.".to_string())
        .block(block);
    f.render_widget(paragraph, rect);
}

fn draw_main_panel<B: Backend>(f: &mut Frame<B>, app: &mut Idle, rect: Rect) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(0)
    .constraints(
        [
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ].as_ref()
    )
    .split(rect);

    draw_sparkline(f, app, chunks[0]);
    draw_graph(f, app, chunks[1]);
    draw_chat(f, app, chunks[2]);    
}

fn draw_sidebar<B: Backend>(f: &mut Frame<B>, app: &Idle, shop: &mut Shop, chunk_rect: Rect) {    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref()
        )
        .split(chunk_rect);

    draw_stats(f, app, chunks[0]);    
    draw_shop(f, app, shop, chunks[1]);
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut Idle, shop: &mut Shop) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25)
            ].as_ref()
        )
        .split(f.size());

    draw_main_panel(f, app, chunks[0]);
    draw_sidebar(f, app, shop, chunks[1]);
}
