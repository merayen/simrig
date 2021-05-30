/** Turns LEDs on and off. Also pulsewidth modulates them **/
use std::io::prelude::*;
use crate::ic::mcp23s17::MCP23S17;

pub const LED_COUNT: usize = 10;
pub const LED_MAX_POWER: u8 = 10;

pub struct LEDController {
	position: u8,

	// Value for each LED. 0 = no light. 127 = 50% light, 255 = yes
	led_power: Vec<u8>,
	led_last_value: Vec<bool>,
}

impl LEDController {
	pub fn new(count: u8) -> LEDController {
		let mut led_power: Vec<u8> = Vec::new();
		let mut led_last_value: Vec<bool> = Vec::new();

		for _ in 0..count {
			led_power.push(0);
			led_last_value.push(false);
		}

		LEDController {
			position: 0,
			led_power: led_power,
			led_last_value: led_last_value,
		}
	}

	/** Calls back the 'func'-method for LEDs that has changed since last time **/
	pub fn update<F>(&mut self, func: F) where F: Fn(usize, bool) {
		self.position += 1;
		self.position %= LED_MAX_POWER + 1;

		let position = &self.position;

		for (i,x) in self.led_power.iter().enumerate() {
			let new_value: bool;
			new_value = x >= position;

			if new_value != self.led_last_value[i] {
				func(i, new_value);
				self.led_last_value[i] = new_value;
			}
		}
	}

	pub fn set(&mut self, led: usize, power: u8) {
		if power > LED_MAX_POWER {
			self.led_power[led] = LED_MAX_POWER;
		} else {
			self.led_power[led] = power;
		}
	}
}
