use crate::pages::{Page, Draw};
use crate::ui::UI;
use crate::fonts::get_7_segment_text;
use crate::util::get_time;

pub struct Main {
	draw: Draw,
}

impl Page for Main {
	fn draw(&mut self) -> Option<&mut Draw>{
		// Gear
		self.draw.rect(35, 10, 30, 10);
		//get_7_segment_text(43, 11, 14, 9, ((get_time() / 1000) % 12) as usize, 1, &mut self.draw);
		get_7_segment_text(43, 11, 14, 9, 8, 1, &mut self.draw);

		// Clutch
		self.draw.rect(70, 10, 30, 3);
		self.draw.text(71, 11, "Clutch");

		// Brake
		self.draw.rect(70, 13, 30, 3);
		self.draw.text(71, 14, "Brake");

		// Throttle
		self.draw.rect(70, 16, 30, 3);
		self.draw.text(71, 17, "Throttle");

		Some(&mut self.draw)
	}
}

impl Main {
	pub fn new() -> Main {
		return Main {
			draw: Draw::new(),
		};
	}
}
