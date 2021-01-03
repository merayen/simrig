pub mod logo;
use crate::ui::{WIDTH, HEIGHT};

pub trait Page {
	fn draw(&mut self) -> &Draw;
}

pub struct Draw {
	text: [char; WIDTH * HEIGHT],
	fg: [u8; WIDTH * HEIGHT],
		bg: [u8; WIDTH * HEIGHT],
}

impl Draw {
	pub fn new() -> Draw {
		return Draw {
			text: [' '; WIDTH * HEIGHT],
			fg: [0; WIDTH * HEIGHT],
			bg: [0; WIDTH * HEIGHT],
		};
	}

	pub fn clear(&mut self) {}

	pub fn text(&mut self, x: usize, y: usize, text: &str) {
		let mut x_pos: usize = x;
		let mut y_pos: usize = y;

		for c in text.chars() {
				if x_pos >= WIDTH || y_pos >= HEIGHT {
					break;
				}

				self.text[x_pos + y_pos * WIDTH] = c;
				x_pos += 1;
		}
	}

	pub fn rect(&mut self, x: usize, y: usize, width: usize, height: usize) {}
	pub fn fg(&mut self, code: u8) {}
	
	pub fn draw(&self) { // TODO create a buffer, then print it in 1 go instead?
		for i in 0..WIDTH*HEIGHT {
			let character = self.text[i];
			let fg = self.fg[i];
			let bg = self.bg[i];
			if fg > 0 {
			} else {
			}
			if bg > 0 {
			} else {
			}
			if i > 0 && i % WIDTH == 0 {
				println!("");
			}
			print!("{}", character);
		}
		println!("\x1B[{}A", HEIGHT);
	}
}
