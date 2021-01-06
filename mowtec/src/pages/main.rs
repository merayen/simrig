use crate::pages::{Page, Draw};
use crate::ui::{UI, WIDTH, HEIGHT};
use crate::fonts::get_7_segment_text;

pub struct Main {
	draw: Draw,
	pub throttle: f32, // 0.0 .. 1.0
	pub brake: f32, // 0.0 .. 1.0
	pub clutch: f32, // 0.0 .. 1.0
	pub gear: i8, // -1 = R, 0 = N, 1 = 1, 2 = 2...
}

impl Page for Main {
	fn draw(&mut self) -> Option<&mut Draw>{
		// Temporary frame to see borders
		self.draw.bg_current = 15;
		self.draw.frame_border(0, 0, WIDTH, HEIGHT);

		// Gear
		self.draw.bg_current = 3;
		self.draw.frame_border(5, 10, 30, 11);

		self.draw.bg_current = 16;
		let gear;
		if self.gear == -1 {
			gear = 12;
		} else if self.gear == 0 {
			gear = 11;
		} else {
			gear = self.gear;
		}
		get_7_segment_text(13, 11, 12, 9, gear as usize, &mut self.draw);

		// Clutch
		self.draw.bg_current = 9;
		self.draw.frame_border(70, 10, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(71, 11, "Clutch");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 11, 28, 1);
		self.draw.bg_current = 123;
		self.draw.rect_bg(71, 11, std::cmp::min(28, (28.0 * self.clutch).round() as usize) as usize, 1);

		// Brake
		self.draw.bg_current = 9;
		self.draw.frame_border(70, 13, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(71, 14, "Brake");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 14, 28, 1);
		self.draw.bg_current = 2;
		self.draw.rect_bg(71, 14, std::cmp::min(28, (28.0 * self.brake).round() as usize) as usize, 1);

		// Throttle
		self.draw.bg_current = 9;
		self.draw.frame_border(70, 16, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 15;
		self.draw.text(71, 17, "Throttle");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 17, 28, 1);
		self.draw.bg_current = 9;
		self.draw.rect_bg(71, 17, std::cmp::min(28, (28.0 * self.throttle).round() as usize) as usize, 1);

		Some(&mut self.draw)
	}
}

impl Main {
	pub fn new() -> Main {
		return Main {
			draw: Draw::new(),
			throttle: 0.0,
			brake: 0.0,
			clutch: 0.0,
			gear: 0,
		};
	}
}
