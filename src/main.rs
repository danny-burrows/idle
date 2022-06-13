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

struct Idle<'a> {
    incrementors: [&'a str; 3],
    incrementors_state: ListState
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

    let items = app.incrementors.map(|f| {ListItem::new(f)});
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunks[1], &mut app.incrementors_state);
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    
    let mut app = Idle {
        incrementors: ["Incrementor 1", "Incrementor 2", "Incrementor 3"],
        incrementors_state: ListState::default()
    };

    app.incrementors_state.select(Some(0));
    
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up => {
                    let current_select = app.incrementors_state.selected();
                    
                    if current_select != None && current_select.unwrap() > 0 {
                        app.incrementors_state.select(Some(current_select.unwrap() - 1));
                    }
                },
                KeyCode::Down => {
                    let current_select = app.incrementors_state.selected();

                    if current_select != None && current_select.unwrap() < 2 {
                        app.incrementors_state.select(Some(current_select.unwrap() + 1));
                    }
                },
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
