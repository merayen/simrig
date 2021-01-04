pub mod logo;
pub mod main;
use crate::ui::{WIDTH, HEIGHT};

pub trait Page {
	fn draw(&mut self) -> Option<&mut Draw>;
}

pub struct Draw {
	text: [char; WIDTH * HEIGHT],
	fg: [u8; WIDTH * HEIGHT],
	bg: [u8; WIDTH * HEIGHT],
	fg_current: u8,
	bg_current: u8,
}

impl Draw {
	pub fn new() -> Draw {
		return Draw {
			text: [' '; WIDTH * HEIGHT],
			fg: [0; WIDTH * HEIGHT],
			bg: [0; WIDTH * HEIGHT],
			fg_current: 0,
			bg_current: 0,
		};
	}

	pub fn clear(&mut self) {
		for x in &mut self.text {
			*x = ' ';
		}
	}

	pub fn text(&mut self, x: usize, y: usize, text: &str) {
		let mut x_pos: usize = x;
		let mut y_pos: usize = y;

		for c in text.chars() {
				if c == '\n' {
					y_pos += 1;
					x_pos = x;
					continue;
				}
				//println!("{}, {}", x_pos, y_pos);

				if x_pos < WIDTH && y_pos < HEIGHT {
					let pos = x_pos + y_pos * WIDTH;
					self.text[pos] = c;
					self.fg[pos] = self.fg_current;
					self.bg[pos] = self.bg_current;
				}

				x_pos += 1;
		}
	}

	pub fn rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
		for i in x..std::cmp::min(x+width, WIDTH) { // Top and bottom line
			self.fg[y * WIDTH + i] = self.fg_current;
			self.bg[y * WIDTH + i] = self.bg_current;
			self.text[y * WIDTH + i] = ' ';

			self.fg[(y + height) * WIDTH + i] = self.fg_current;
			self.bg[(y + height) * WIDTH + i] = self.bg_current;
			self.text[(y + height) * WIDTH + i] = ' ';
		}

		for i in y+1..std::cmp::min(y+height, HEIGHT) { // Left and right line
			let pos = i * WIDTH + x;
			self.fg[pos] = self.fg_current;
			self.bg[pos] = self.bg_current;
			self.text[pos] = '|';

			let pos2 = i * WIDTH + x + width - 1;
			self.fg[pos2] = self.fg_current;
			self.bg[pos2] = self.bg_current;
			self.text[pos2] = '|';
		}
	}

	pub fn fg(&mut self, code: u8) {
		self.fg_current = code;	
	}
	
	pub fn bg(&mut self, code: u8) {
		self.bg_current = code;
	}
	
	pub fn draw(&mut self) { // TODO create a buffer, then print it in 1 go instead?
		let mut output: Vec<char> = Vec::with_capacity(WIDTH * HEIGHT);

		for i in 0..WIDTH*HEIGHT {
			let character = self.text[i];
			let fg = self.fg[i];
			let bg = self.bg[i];

			if i > 0 && i % WIDTH == 0 {
				output.push('\n');
			}

			if fg > 0 { // TODO Check if same color repeats
				output.push('\x1B');
				output.push('[');
				output.push('3');
				output.push('8');
				output.push(';');
				output.push('5');
				output.push(';');
				output.push(std::char::from_digit(fg as u32, 10).unwrap());
				output.push('m');
			} else {
			}

			if bg > 0 {
				output.push('\x1B');
				output.push('[');
				output.push('4');
				output.push('8');
				output.push(';');
				output.push('5');
				output.push(';');
				output.push(std::char::from_digit(bg as u32, 10).unwrap());
				output.push('m');
			} else {
			}

			output.push(character);
		}

		let string: String = output.iter().collect();

		print!("{}", string);
		println!("\x1B[{}A\x1B[K", HEIGHT);

		self.fg_current = 0;
		self.bg_current = 0;
	}
}
