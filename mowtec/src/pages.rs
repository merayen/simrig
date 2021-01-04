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
	fg_stack: Vec<u8>,
	bg_stack: Vec<u8>,
}

//pub trait DrawTrait {
//	fn plot_bg(&mut self, x: usize, y: usize, color: u8);
//	fn clear(&mut self);
//	fn text(&mut self, x: usize, y: usize, text: &str);
//	fn rect(&mut self, x: usize, y: usize, width: usize, height: usize);
//	fn fg(&mut self, code: u8);
//	fn bg(&mut self, code: u8);
//	fn draw(&mut self);
//}

impl Draw {
	pub fn new() -> Draw {
		return Draw {
			text: [' '; WIDTH * HEIGHT],
			fg: [0; WIDTH * HEIGHT],
			bg: [0; WIDTH * HEIGHT],
			fg_stack: Vec::new(),
			bg_stack: Vec::new(),
		};
	}

	pub fn clear(&mut self) {
		for x in &mut self.text {
			*x = ' ';
		}
	}

	pub fn current_fg(&self) -> u8 {
		let result = self.fg_stack.last();
		if result.is_none() {
			return 0;
		} else {
			return *result.unwrap();
		}
	}

	pub fn current_bg(&self) -> u8{
		*self.bg_stack.last().unwrap_or_else(||&0)
	}

	pub fn push_fg(&mut self, color: u8) {
		self.fg_stack.push(color);
	}

	pub fn push_bg(&mut self, color: u8) {
		self.bg_stack.push(color);
	}

	pub fn pop_fg(&mut self) {
		self.fg_stack.pop();
	}

	pub fn pop_bg(&mut self) {
		self.bg_stack.pop();
	}

	pub fn plot_bg(&mut self, x: usize, y: usize, color: u8) {
		self.bg[x + WIDTH * y] = color;
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
					self.fg[pos] = self.current_fg();
					self.bg[pos] = self.current_bg();
				}

				x_pos += 1;
		}
	}

	pub fn rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
		for i in x..std::cmp::min(x+width, WIDTH) { // Top and bottom line
			self.fg[y * WIDTH + i] = self.current_fg();
			self.bg[y * WIDTH + i] = self.current_bg();
			self.text[y * WIDTH + i] = ' ';

			self.fg[(y + height) * WIDTH + i] = self.current_fg();
			self.bg[(y + height) * WIDTH + i] = self.current_bg();
			self.text[(y + height) * WIDTH + i] = ' ';
		}

		for i in y+1..std::cmp::min(y+height, HEIGHT) { // Left and right line
			let pos = i * WIDTH + x;
			self.fg[pos] = self.current_fg();
			self.bg[pos] = self.current_bg();
			self.text[pos] = '|';

			let pos2 = i * WIDTH + x + width - 1;
			self.fg[pos2] = self.current_fg();
			self.bg[pos2] = self.current_bg();
			self.text[pos2] = '|';
		}
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
				output.push('\x1B');
				output.push('[');
				output.push('4');
				output.push('8');
				output.push(';');
				output.push('5');
				output.push(';');
				output.push('0');
				output.push('m');
			}

			output.push(character);
		}

		let string: String = output.iter().collect();

		print!("{}", string);
		println!("\x1B[{}A\x1B[K", HEIGHT);

		self.fg_stack.clear();
		self.bg_stack.clear();
	}
}
