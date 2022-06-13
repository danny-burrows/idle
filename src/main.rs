use std::{io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn ui<B: Backend>(f: &mut Frame<B>) {
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
 }


fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
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
