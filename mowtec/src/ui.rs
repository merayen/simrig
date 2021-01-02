// The UI! Amazingly great-looking terminal UI!

const WIDTH: usize = 100;
const HEIGHT: usize = 30;


pub struct UI {
	buffer: [char; WIDTH * HEIGHT],
}

impl UI {
	pub fn new() -> UI {
		return UI {
			buffer: [' '; WIDTH * HEIGHT]
		};
	}

	fn rect(&self) {
	}
}


