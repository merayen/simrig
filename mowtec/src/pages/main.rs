use crate::pages::{Page, Draw};
use crate::ui::UI;
use crate::fonts::get_7_segment_text;
use crate::util::get_time;

pub struct Main {
	draw: Draw,
}

impl Page for Main {
	fn draw(&mut self) -> Option<&Draw>{
		self.draw.fg(0);
		self.draw.rect(35, 10, 30, 10);
		//self.draw.text(36, 11, get_number_text(0).as_str());
		self.draw.text(36, 11, get_7_segment_text(8,9,((get_time() / 1000) % 12) as usize).as_str());
		Some(&self.draw)
	}
}

impl Main {
	pub fn new() -> Main {
		return Main {
			draw: Draw::new(),
		};
	}
}
