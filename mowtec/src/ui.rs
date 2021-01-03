// The UI! Amazingly great-looking terminal UI!

use crate::pages::Page;

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 30;


pub struct UI {}

impl UI {
	pub fn new() -> UI {
		UI {}
	}

	pub fn draw(&self, page: &mut impl Page) {
		let to_draw = page.draw();
		if to_draw.is_none() {
			return; // Nothing new has been drawn, we skip
		}
		to_draw.unwrap().draw();
	}
}


