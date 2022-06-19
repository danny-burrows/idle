use std::io::{Error, ErrorKind};
use tui::{widgets::ListState, style::Color};

pub enum ShopItem {
    IncrementorPurchase {
        text: String,
        price: f64,
        colour: Color,
        incrementor_index: usize,
    },
    IncrementorUpgrade {
        text: String,
        price: f64,
        colour: Color,
        incrementor_index: usize,
    },
}

pub struct Shop {
    pub items: Vec<ShopItem>,
    pub state: ListState
}

impl Shop {
    pub fn next(&mut self) {
        if let Some(current_select) = self.state.selected() {
            if current_select < self.items.len() - 1 {
                self.state.select(Some(current_select + 1));
            }
        }
    }

    pub fn prev(&mut self) {
        if let Some(current_select) = self.state.selected() {
            if current_select > 0 {
                self.state.select(Some(current_select - 1));
            }
        }
    }

    pub fn _get_selected(&self) -> Result<&ShopItem, Error> {
        if let Some(selected_i) = self.state.selected() {
            if let Some(shop_item) = self.items.get(selected_i) {
                return Ok(shop_item);
            }
        }
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }

    pub fn get_mut_selected_with_index(&mut self) -> Result<(usize, &mut ShopItem), Error> {
        if let Some(selected_i) = self.state.selected() {
            if let Some(shop_item) = self.items.get_mut(selected_i) {
                return Ok((selected_i, shop_item));
            }
        }
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }
}

impl Default for Shop {   
    fn default() -> Self {
        Shop {
            items: vec![],
            state: ListState::default()
        }
    }
}
