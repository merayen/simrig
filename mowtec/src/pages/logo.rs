use crate::pages::{Page, Draw};
use crate::ui::UI;
use crate::util::get_time;

pub struct Logo {
	draw: Draw,
}

impl Page for Logo {
	fn draw(&mut self) -> Option<&mut Draw> {
		let tid = get_time() as f64 / 1000.0;

		self.draw.bg_current = 0;
		self.draw.clear();

		self.draw.fg_current = 123;
		self.draw.text(((tid * 5.0).sin() * 40.0 + 45.0) as usize, 14, "MowTec!");

		Some(&mut self.draw)
	}
}

impl Logo {
	pub fn new() -> Logo {
		return Logo {
			draw: Draw::new(),
		};
	}
}
