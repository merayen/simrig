use crate::pages::{Page, Draw};
use crate::ui::{UI, WIDTH, HEIGHT};
use crate::fonts::get_7_segment_text;

const TIRE_COLOR_TEMPS: [u8; 7] = [
	93, // Cold
	58,
	48,
	74, // Good temp
	11, // A bit warm
	1,
	9, // Overheating
];

pub struct Main {
	draw: Draw,
	pub rpm: f32,
	pub throttle: f32, // 0.0 .. 1.0
	pub brake: f32, // 0.0 .. 1.0
	pub clutch: f32, // 0.0 .. 1.0
	pub gear: i8, // -1 = R, 0 = N, 1 = 1, 2 = 2...
	pub tire_wear: [f32; 4],
	pub tire_temperature: [f32; 4],
}

impl Page for Main {
	fn draw(&mut self) -> Option<&mut Draw>{
		// Temporary frame to see borders
		//self.draw.fg_current = 15;
		//self.draw.frame_border(0, 0, WIDTH, HEIGHT);

		// Unnecessary RPM meter on top
		self.draw.bg_current = 0;
		self.draw.rect_bg(0, 0, WIDTH, 2);
		self.draw.bg_current = 3;
		self.draw.rect_bg(0, 0, (self.rpm * WIDTH as f32).round() as usize, 2);

		// Gear
		self.draw.fg_current = 80;
		self.draw.frame_border(35, 5, 30, 21);

		self.draw.bg_current = 16;
		let gear;
		if self.gear == -1 {
			gear = 12;
		} else if self.gear == 0 {
			gear = 11;
		} else {
			gear = self.gear;
		}
		get_7_segment_text(43, 6, 14, 19, gear as usize, &mut self.draw);

		// Clutch
		self.draw.fg_current = 80;
		self.draw.frame_border(70, 5, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 0;
		self.draw.text(71, 6, "Clutch");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 6, 28, 1);
		self.draw.bg_current = 123;
		self.draw.rect_bg(71, 6, std::cmp::min(28, (28.0 * self.clutch).round() as usize) as usize, 1);

		// Brake
		self.draw.fg_current = 80;
		self.draw.frame_border(70, 9, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 0;
		self.draw.text(71, 10, "Brake");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 10, 28, 1);
		self.draw.bg_current = 9;
		self.draw.rect_bg(71, 10, std::cmp::min(28, (28.0 * self.brake).round() as usize) as usize, 1);

		// Throttle
		self.draw.fg_current = 80;
		self.draw.frame_border(70, 13, 30, 3);
		self.draw.bg_current = 0;
		self.draw.fg_current = 0;
		self.draw.text(71, 14, "Throttle");

		self.draw.bg_current = 0;
		self.draw.rect_bg(71, 14, 28, 1);
		self.draw.bg_current = 2;
		self.draw.rect_bg(71, 14, std::cmp::min(28, (28.0 * self.throttle).round() as usize) as usize, 1);


		// Tires
		for tire_y in 0..2 {
			for tire_x in 0..2 {
				self.draw.fg_current = 80;
				self.draw.frame_border(tire_x * 12, 5 + 12 * tire_y, 10, 10);

				// Clear out tire background
				self.draw.bg_current = 0;
				self.draw.rect_bg(1 + tire_x * 12, 6 + 12 * tire_y, 8, 8);

				// Calc and draw the tire column
				self.draw.bg_current = TIRE_COLOR_TEMPS[(self.tire_temperature[tire_y * 2 + tire_x] * (TIRE_COLOR_TEMPS.len() - 1) as f32).round() as usize];
				let height = (self.tire_wear[tire_y * 2 + tire_x] * 8.0).round() as usize;
				self.draw.rect_bg(1 + tire_x * 12, 6 + 12 * tire_y + height, 8, 8-height);
			}
		}

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
			tire_wear: [0.0; 4],
			tire_temperature: [0.0; 4],
			rpm: 0.0,
		};
	}
}
