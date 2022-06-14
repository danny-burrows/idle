use std::{io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, List, ListItem, ListState, Borders},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
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
    incrementors: Incrementors
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Idle) {
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
        
    let block = Block::default()
        .title("Main Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    
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
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    
    let mut app = Idle {
        incrementors: Incrementors { 
            list: vec!["Incrementor 1".to_string(), "Incrementor 2".to_string(), "Incrementor 3".to_string()], 
            state: ListState::default() 
        }
    };

    app.incrementors.state.select(Some(0));
    
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up => app.incrementors.prev(),
                KeyCode::Down => app.incrementors.next(),
                _ => {}
            }

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
