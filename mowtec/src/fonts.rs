use crate::pages::Draw;

const SEVEN_SEGMENT: [u8; 12] = [
	0b1111110, // 0
	0b0110000, // 1
	0b1101101, // 2
	0b1111001, // 3
	0b0110011, // 4
	0b1011011, // 5
	0b1011111, // 6
	0b1110000, // 7
	0b1111111, // 8
	0b1111011, // 9
	0b0000000, // Nothing
	0b0000001, // -
];

pub fn get_7_segment_text(x: usize, y: usize, width: usize, height: usize, number: usize, color: u8, draw: &mut Draw) {
	let source = SEVEN_SEGMENT[number];

	draw.push_bg(0);
	draw.rect_bg(x, y, width, height);
	draw.pop_bg();

	if source & 64 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y, color);
		}
	}

	if source & 32 > 0 {
		for i in 1..height/2 {
			draw.plot_bg(x + width - 1, i + y, color);
		}
	}

	if source & 16 > 0 {
		for i in height/2+1..height-1 {
			draw.plot_bg(x + width - 1, i + y, color);
		}
	}

	if source & 8 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y + height - 1, color);
		}
	}

	if source & 4 > 0 {
		for i in height/2+1..height-1 {
			draw.plot_bg(x, y+i, color);
		}
	}

	if source & 2 > 0 {
		for i in 1..height/2 {
			draw.plot_bg(x, y+i, color);
		}
	}

	if source & 1 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y + (height-1) / 2, color);
		}
	}
}
