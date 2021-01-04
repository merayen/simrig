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
		let result = self.bg_stack.last();
		if result.is_none() {
			return 0;
		} else {
			return *result.unwrap();
		}
	}

	pub fn push_fg(&mut self, color: u8) {
		if self.fg_stack.len() >= 10 {
			panic!("Forgotten to call pop_fg()? Or are you perhaps drawing a rainbow recursively?");
		}
		self.fg_stack.push(color);
	}

	pub fn push_bg(&mut self, color: u8) {
		if self.bg_stack.len() >= 10 {
			panic!("Forgotten to call pop_bg()? Or are you perhaps drawing a rainbow recursively?");
		}
		self.bg_stack.push(color);
	}

	pub fn pop_fg(&mut self) {
		self.fg_stack.pop();
	}

	pub fn pop_bg(&mut self) {
		self.bg_stack.pop();
	}

	pub fn plot_fg(&mut self, x: usize, y: usize, color: u8) {
		self.fg[x + WIDTH * y] = color;
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

				if x_pos < WIDTH && y_pos < HEIGHT {
					let pos = x_pos + y_pos * WIDTH;
					self.text[pos] = c;
					self.fg[pos] = self.current_fg();
					self.bg[pos] = self.current_bg();
				}

				x_pos += 1;
		}
	}

	pub fn frame(&mut self, x: usize, y: usize, width: usize, height: usize) {
		let color = self.current_bg();
		for i in x..x+width { // Top and bottom line
			self.bg[y * WIDTH + i] = color;
			self.bg[(y + height - 1) * WIDTH + i] = color;
		}

		for i in y+1..std::cmp::min(y+height, HEIGHT) { // Left and right line
			self.bg[i * WIDTH + x] = color;
			self.bg[i * WIDTH + x + width - 1] = color;
		}
	}

	pub fn rect_bg(&mut self, x: usize, y: usize, width: usize, height: usize) {
		let color = self.current_bg();
		for i in y..std::cmp::min(y+height, HEIGHT) {
			for j in x..std::cmp::min(x+width, WIDTH) {
				self.bg[i * WIDTH + j] = color;
			}
		}
	}

	pub fn frame_border(&mut self, x: usize, y: usize, width: usize, height: usize) {

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
				output.push(std::char::from_digit(fg as u32, 10).unwrap());
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
				output.push(std::char::from_digit(bg as u32, 10).unwrap());
				output.push('m');
			} else {
				term_clear_bg(&mut output);
			}

			output.push(character);
		}

		let string: String = output.iter().collect();

		print!("{}", string);
		println!("\x1B[{}A\x1B[K", HEIGHT);

		assert_eq!(self.fg_stack.len(), 0);
		assert_eq!(self.bg_stack.len(), 0);

		self.fg_stack.clear();
		self.bg_stack.clear();
	}

}
