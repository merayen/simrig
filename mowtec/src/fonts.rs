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

pub fn get_7_segment_text(x: usize, y: usize, width: usize, height: usize, number: usize, draw: &mut Draw) {
	let source = SEVEN_SEGMENT[number];

	let active_color = draw.bg_current;
	draw.bg_current = 0;
	draw.rect_bg(x, y, width, height);
	draw.bg_current = active_color;

	if source & 64 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y);
		}
	}

	if source & 32 > 0 {
		for i in 1..height/2 {
			draw.plot_bg(x + width - 1, i + y);
		}
	}

	if source & 16 > 0 {
		for i in height/2+1..height-1 {
			draw.plot_bg(x + width - 1, i + y);
		}
	}

	if source & 8 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y + height - 1);
		}
	}

	if source & 4 > 0 {
		for i in height/2+1..height-1 {
			draw.plot_bg(x, y+i);
		}
	}

	if source & 2 > 0 {
		for i in 1..height/2 {
			draw.plot_bg(x, y+i);
		}
	}

	if source & 1 > 0 {
		for i in 1..width-1 {
			draw.plot_bg(x + i, y + (height-1) / 2);
		}
	}
}
