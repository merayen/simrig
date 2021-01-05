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
	pub fg_current: u8,
	pub bg_current: u8,
}

impl Draw {
	pub fn new() -> Draw {
		return Draw {
			text: [' '; WIDTH * HEIGHT],
			fg: [0; WIDTH * HEIGHT],
			bg: [0; WIDTH * HEIGHT],
			fg_current: 15,
			bg_current: 0,
		};
	}

	pub fn clear(&mut self) {
		for x in &mut self.text {
			*x = ' ';
		}
	}

	pub fn plot_fg(&mut self, x: usize, y: usize) {
		self.fg[x + WIDTH * y] = self.fg_current;
	}

	pub fn plot_bg(&mut self, x: usize, y: usize) {
		self.bg[x + WIDTH * y] = self.bg_current;
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

				if x_pos < WIDTH && y_pos < HEIGHT {
					let pos = x_pos + y_pos * WIDTH;
					self.text[pos] = c;
					self.fg[pos] = self.fg_current;
					self.bg[pos] = self.bg_current;
				}

				x_pos += 1;
		}
	}

	pub fn frame(&mut self, x: usize, y: usize, width: usize, height: usize) {
		for i in x..x+width { // Top and bottom line
			self.bg[y * WIDTH + i] = self.bg_current;
			self.bg[(y + height - 1) * WIDTH + i] = self.bg_current;
		}

		for i in y+1..std::cmp::min(y+height, HEIGHT) { // Left and right line
			self.bg[i * WIDTH + x] = self.bg_current;
			self.bg[i * WIDTH + x + width - 1] = self.bg_current;
		}
	}

	pub fn rect_bg(&mut self, x: usize, y: usize, width: usize, height: usize) {
		for i in y..std::cmp::min(y+height, HEIGHT) {
			for j in x..std::cmp::min(x+width, WIDTH) {
				self.bg[i * WIDTH + j] = self.bg_current;
			}
		}
	}

	pub fn frame_border(&mut self, x: usize, y: usize, width: usize, height: usize) {
		for i in x..x+width { // Top and bottom line
			self.text[y * WIDTH + i] = 'X';
			self.text[(y + height - 1) * WIDTH + i] = 'X';
		}

		for i in y+1..std::cmp::min(y+height, HEIGHT) { // Left and right line
			self.text[i * WIDTH + x] = 'X';
			self.text[i * WIDTH + x + width - 1] = 'X';
		}
	}

	pub fn draw(&mut self) { // TODO create a buffer, then print it in 1 go instead?
		let mut output: Vec<char> = Vec::with_capacity(WIDTH * HEIGHT);

		fn term_clear_fg(output: &mut Vec<char>) {
			output.push('\x1B');
			output.push('[');
			output.push('3');
			output.push('8');
			output.push(';');
			output.push('5');
			output.push(';');
			output.push('1');
			output.push('5');
			output.push('m');
		};

		fn term_clear_bg(output: &mut Vec<char>) {
			output.push('\x1B');
			output.push('[');
			output.push('4');
			output.push('8');
			output.push(';');
			output.push('5');
			output.push(';');
			output.push('0');
			output.push('m');
		};


		for i in 0..WIDTH*HEIGHT {
			let character = self.text[i];
			let fg = self.fg[i];
			let bg = self.bg[i];

			if i > 0 && i % WIDTH == 0 {
				term_clear_bg(&mut output); // So that lines on right side does not span over
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
				output.push(std::char::from_digit((fg % 10) as u32, 10).unwrap());
				if fg >= 10 {
					output.push(std::char::from_digit(((fg/10) % 10) as u32, 10).unwrap());
				}
				output.push('m');
			} else {
				term_clear_fg(&mut output);
			}

			if bg > 0 {
				output.push('\x1B');
				output.push('[');
				output.push('4');
				output.push('8');
				output.push(';');
				output.push('5');
				output.push(';');
				output.push(std::char::from_digit((bg % 10) as u32, 10).unwrap());
				if bg >= 10 {
					output.push(std::char::from_digit(((bg/10) % 10) as u32, 10).unwrap());
				}
				output.push('m');
			} else {
				term_clear_bg(&mut output);
			}

			output.push(character);
		}

		let string: String = output.iter().collect();

		print!("{}", string);
		print!("\x1B[{}A\x1B[K", HEIGHT);


		self.fg_current = 15;
		self.bg_current = 0;
	}

}
