use tui::style::Color;

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
        0.0
    }
}

impl Default for Incrementor {
    fn default() -> Self {
        Incrementor {
            name: "Incrementor",
            colour: Color::LightBlue,
            
            // State
            unlocked: false,
            increment_by: 0.002,
            max_clicks: 1.0,
            price: 1.0,
            price_mult: 1.4,

            // Tracking variables
            clicks: 0.0,
            spare: 0.0,
            total_earned: 0.0,
        }
    }
}
