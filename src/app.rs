use std::{
    io, 
    time::{Duration, Instant}
};
use tui::{
    backend::{Backend},
    widgets::ListState,
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use crate::ui::ui;

pub struct Incrementors {
    pub list: Vec<String>,
    pub state: ListState
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

pub struct Idle {
    pub total_clicks: u64,
    pub inc: u64,
    pub sparkline_max_length: usize,
    pub sparkline_data: Vec<u64>,
    pub graph_data: Vec<u64>,
    pub incrementors: Incrementors
}

impl Idle {
    fn on_tick(&mut self) {
        self.sparkline_data.push(self.inc);
        
        self.graph_data.push(self.total_clicks);
        
        if self.graph_data.len() > 1000 {
            self.graph_data.remove(0);
        }

        self.inc = 0;
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    
    let tick_rate = Duration::from_millis(250);

    let mut app = Idle {
        total_clicks: 0,
        inc: 0,
        sparkline_max_length: 0,
        sparkline_data: vec![],
        graph_data: vec![],
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
                    KeyCode::Char('s') => {
                        if app.total_clicks >= 10 {
                            app.total_clicks -= 10;
                        }
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
