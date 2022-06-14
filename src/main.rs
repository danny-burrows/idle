use std::{
    io, 
    time::{Duration, Instant}, fmt::format
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, List, ListItem, ListState, Borders, LineGauge, Sparkline, Paragraph},
    layout::{Layout, Alignment, Constraint, Direction},
    style::{Style, Modifier, Color},
    symbols,
    Terminal,
    Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct Incrementors {
    list: Vec<String>,
    state: ListState
}

impl Incrementors {
    fn next(&mut self) {
        if let Some(current_select) = self.state.selected() {
            if current_select < self.list.len() - 1 {
                self.state.select(Some(current_select + 1));
            }
        }
    }

    fn prev(&mut self) {
        if let Some(current_select) = self.state.selected() {
            if current_select > 0 {
                self.state.select(Some(current_select - 1));
            }
        }
    }
}

struct Idle {
    total_clicks: u64,
    inc: u64,
    sparkline_max_length: usize,
    sparkline_data: Vec<u64>,
    incrementors: Incrementors
}

impl Idle {
    fn on_tick(&mut self) {
        self.sparkline_data.push(self.inc);
        self.inc = 0;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Idle) {
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


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    
    let tick_rate = Duration::from_millis(250);

    let mut app = Idle {
        total_clicks: 0,
        inc: 0,
        sparkline_max_length: 0,
        sparkline_data: vec![],
        incrementors: Incrementors { 
            list: vec!["Incrementor 1".to_string(), "Incrementor 2".to_string(), "Incrementor 3".to_string()], 
            state: ListState::default() 
        }
    };

    app.incrementors.state.select(Some(0));
    
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
    
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.incrementors.prev(),
                    KeyCode::Down => app.incrementors.next(),
                    KeyCode::Enter => {
                        let i = app.incrementors.state.selected().unwrap() as u64 + 1; 

                        app.inc += i;
                        app.total_clicks += i;
                    },
                    _ => {}
                }
    
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}


fn main() -> Result<(), io::Error> {

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
