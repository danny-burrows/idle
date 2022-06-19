use std::{
    io::{self, Error}, 
    time::{Duration, Instant}
};
use tui::{
    backend::Backend,
    style::Color,
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use crate::{
    ui::draw_ui,
    incrementor::Incrementor,
    shop::{Shop, ShopItem}
};

const TICK_RATE: Duration = Duration::from_millis(15);

pub struct Idle {
    pub total_clicks: f64,
    pub all_time_total_clicks: f64,
    pub inc: u64,
    pub sparkline_max_length: usize,
    pub sparkline_data: Vec<u64>,
    pub graph_data: Vec<f64>,
    pub incrementors: [Incrementor; 5],
    pub shop: Shop
}

impl Idle {
    fn on_tick(&mut self) {
        for incrementor in self.incrementors.iter_mut() {
            
            if !incrementor.unlocked {continue;}
            
            let i = incrementor.tick();

            self.total_clicks += i;
            self.all_time_total_clicks += i;
            self.inc += i.round() as u64;
        }

        if self.inc > 0 {
            self.sparkline_data.push(self.inc);
        }
        
        self.graph_data.push(self.total_clicks);
        
        if self.graph_data.len() > 1000 {
            self.graph_data.remove(0);
        }

        self.inc = 0;
    }
}

fn purchese_shop_item(app: &mut Idle) -> Result<(), Error> {
    let mut remove_shop_item = (false, 0);
    let mut new_shop_items: Vec<ShopItem> = vec![];
    
    // Get selected shop item.
    let (indx, selected) = app.shop.get_mut_selected_with_index()?;

    match selected {
        ShopItem::IncrementorPurchase{ text: _, price, colour, incrementor_index} => {
            if let Some(i) = app.incrementors.get_mut(*incrementor_index) {
                
                // If we can afford it; pay for it and unlock.
                if app.total_clicks >= *price {
                    app.total_clicks -= *price;
                    i.unlocked = true;
                
                    remove_shop_item = (true, indx);
                    new_shop_items.push(
                        ShopItem::IncrementorUpgrade { 
                            text: format!("Upgrade {}", i.name), 
                            price: *price * 2.0,
                            colour: *colour,
                            incrementor_index: *incrementor_index 
                        }
                    );
                }
            }
        }
        ShopItem::IncrementorUpgrade{ text: _, price, colour:_, incrementor_index} => {
            if let Some(i) = app.incrementors.get_mut(*incrementor_index) {

                // If we can afford it; do the upgrade.
                if app.total_clicks >= *price {
                    app.total_clicks -= *price;

                    i.increment_by *= 1.25;
                    i.max_clicks   *= 1.22;

                    *price *= i.price_mult;
                }
            }
        }
    }

    // Remove shop items if needed...
    if remove_shop_item.0 {
        app.shop.items.remove(remove_shop_item.1);
    }

    // Add any new shop items to the shop...
    app.shop.items.append(&mut new_shop_items);

    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {

    let mut app = Idle {
        total_clicks: 1.0,
        all_time_total_clicks: 1.0,
        inc: 0,
        sparkline_max_length: 0,
        sparkline_data: vec![],
        graph_data: vec![],
        incrementors: [
                Incrementor::default(),
                Incrementor {
                    name: "Better Incrementor",
                    colour: Color::LightGreen,

                    increment_by: 0.008,
                    max_clicks: 5.0,
                    price: 10.0,
                    price_mult: 1.4,
                    ..Default::default()
                },
                Incrementor {
                    name: "Improved Incrementor",
                    colour: Color::Yellow,

                    increment_by: 0.1,
                    max_clicks: 20.0,
                    price: 100.0,
                    price_mult: 1.4,

                    ..Default::default()
                },
                Incrementor {
                    name: "Super Incrementor",
                    colour: Color::LightRed,
                    
                    increment_by: 0.1,
                    max_clicks: 50.0,
                    price: 250.0,
                    price_mult: 1.3,
                    ..Default::default()
                },
                Incrementor {
                    name: "God Mode.",
                    colour: Color::LightMagenta,

                    increment_by: 1.0,
                    max_clicks: 100.0,
                    price: 1000.0,
                    price_mult: 2.0,
                    ..Default::default()
            }],
            shop: Shop::default()
    };

    // Add all incrementors to the shop list.
    app.shop.items.append(&mut app.incrementors.iter().enumerate().map(|(i, inc)| 
        ShopItem::IncrementorPurchase {
            text: inc.name.to_string(),
            price: inc.price,
            colour: inc.colour,
            incrementor_index: i,
        }
    ).collect());
    app.shop.state.select(Some(0));
    
    // Main event loop.
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| draw_ui(frame, &mut app))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.shop.prev(),
                    KeyCode::Down => app.shop.next(),
                    KeyCode::Enter => purchese_shop_item(&mut app)?,
                    _ => {}
                }
    
            }
        }
        if last_tick.elapsed() >= TICK_RATE {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
