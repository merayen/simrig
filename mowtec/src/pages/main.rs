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
		self.draw.bg_current = 3;
		self.draw.frame(5, 10, 30, 11);

		self.draw.bg_current = 16;
		get_7_segment_text(13, 11, 12, 9, ((get_time() / 1000) % 12) as usize, &mut self.draw);

		// Clutch
		self.draw.bg_current = 9;
		self.draw.frame(40, 10, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(41, 11, "Clutch");

		// Brake
		self.draw.bg_current = 9;
		self.draw.frame(40, 13, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(41, 14, "Brake");

		// Throttle
		self.draw.bg_current = 9;
		self.draw.frame(40, 16, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(41, 17, "Throttle");

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
