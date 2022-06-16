use std::{
    io, 
    time::{Duration, Instant}
};
use tui::{
    backend::{Backend},
    widgets::ListState,
    style::Color,
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use crate::ui::draw_ui;

pub struct Incrementor {
    pub name: &'static str,
    pub colour: Color,

    pub clicks: f64,
    pub max_clicks: f64,

    pub increment_by: f64,

    pub spare: f64,

    // Stats
    pub total_earned: f64,

    pub unlocked: bool,
    pub price: f64,
    pub price_mult: f64
}

impl Incrementor {
    // Return value is clicks to add to global pool.
    pub fn tick(& mut self) -> f64 {
        self.clicks += self.increment_by;

        // Check for overflow.
        if self.clicks >= self.max_clicks {
            self.spare += self.clicks - self.max_clicks;
        
            let c = self.clicks;

            self.clicks = 0.0;
            
            self.total_earned += c;
            return c;
        }
        return 0.0;
    }
}

pub struct Incrementors {
    pub list: [Incrementor; 5],
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
    pub total_clicks: f64,
    pub all_time_total_clicks: f64,
    pub inc: u64,
    pub sparkline_max_length: usize,
    pub sparkline_data: Vec<u64>,
    pub graph_data: Vec<f64>,
    pub incrementors: Incrementors
}

impl Idle {
    fn on_tick(&mut self) {

        for incrementor in self.incrementors.list.iter_mut() {
            
            if !incrementor.unlocked {continue;}
            
            let i = incrementor.tick();

            self.total_clicks += i;
            self.all_time_total_clicks += i;
            self.inc += i.round() as u64;
        }

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
        total_clicks: 1.0,
        all_time_total_clicks: 1.0,
        inc: 0,
        sparkline_max_length: 0,
        sparkline_data: vec![],
        graph_data: vec![],
        incrementors: Incrementors { 
            list: [
                Incrementor {
                    name: "Incrementor",
                    colour: Color::LightBlue,
                    unlocked: true,

                    clicks:0.0,
                    spare: 0.0,
                    increment_by: 0.1,
                    max_clicks: 1.0,
                    total_earned: 0.0,
                    price: 1.0,
                    price_mult: 1.5
                },
                Incrementor {
                    name: "Better Incrementor",
                    colour: Color::LightGreen,
                    unlocked: false,

                    clicks:0.0,
                    spare: 0.0,
                    increment_by: 0.5,
                    max_clicks: 5.0,
                    total_earned: 0.0,
                    price: 100.0,
                    price_mult: 1.5
                },
                Incrementor {
                    name: "Improved Incrementor",
                    colour: Color::Yellow,
                    unlocked: false,

                    clicks:0.0,
                    spare: 0.0,
                    increment_by: 1.5,
                    max_clicks: 10.0,
                    total_earned: 0.0,
                    price: 1000.0,
                    price_mult: 2.0
                },
                Incrementor {
                    name: "Super Incrementor",
                    colour: Color::LightRed,
                    unlocked: false,
                    
                    clicks:0.0,
                    spare: 0.0,
                    increment_by: 3.0,
                    max_clicks: 50.0,
                    total_earned: 0.0,
                    price: 5000.0,
                    price_mult: 2.0
                },
                Incrementor {
                    name: "God Mode.",
                    colour: Color::LightMagenta,
                    unlocked: false,

                    clicks:0.0,
                    spare: 0.0,
                    increment_by: 10.0,
                    max_clicks: 100.0,
                    total_earned: 0.0,
                    price: 50000.0,
                    price_mult: 2.5
                }],
            state: ListState::default() 
        }
    };

    app.incrementors.state.select(Some(0));
    
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| draw_ui(frame, &mut app))?;

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
                        
                        // Get selected incrementor.
                        let i = app.incrementors.state.selected().unwrap(); 
                        let incrementor = app.incrementors.list.get_mut(i).unwrap();

                        // Attempt purchase.
                        if app.total_clicks >= incrementor.price {
                            app.total_clicks -= incrementor.price;

                            if incrementor.unlocked {
                                incrementor.increment_by *= 1.23;
                                incrementor.max_clicks *= 1.2;
                                
                                incrementor.price = incrementor.price * incrementor.price_mult;
                            } else {
                                incrementor.unlocked = true;
                            }
                        }

                    },
                    KeyCode::Char('s') => {
                        if app.total_clicks >= 10.0 {
                            app.total_clicks -= 10.0;
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
