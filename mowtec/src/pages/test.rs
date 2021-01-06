use crate::pages::{Page, Draw};
use crate::ui::{UI, WIDTH, HEIGHT};


pub struct Test {
	draw: Draw,
}

impl Page for Test {
	fn draw(&mut self) -> Option<&mut Draw>{
		// Color pallette
		for i in 0..=255 {
			self.draw.fg_current = i;
			self.draw.text(i as usize % WIDTH, i as usize / WIDTH, &(i % 10).to_string());
		}

		Some(&mut self.draw)
	}
}

impl Test {
	pub fn new() -> Test {
		return Test {
			draw: Draw::new(),
		};
	}
}
