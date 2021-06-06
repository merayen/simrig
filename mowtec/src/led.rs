/** Turns LEDs on and off. Also pulsewidth modulates them **/
use std::io::prelude::*;
use crate::ic::mcp23s17::MCP23S17;

pub const LED_COUNT: usize = 10;
pub const LED_MAX_POWER: u8 = 10;

pub struct LEDController {
	hz: u32,
	steps: u8,
	step_duration: u64, // Microseconds

	// Value for each LED. 0.0 = no light. 0.5 = 50% light, 1.0 = yes
	led_power: Vec<f32>,
}

impl LEDController {
	pub fn new(count: usize, steps: u8, hz: u32) -> LEDController {
		let mut led_power: Vec<f32> = Vec::new();

		for _ in 0..count {
			led_power.push(0f32);
		}

		LEDController {
			hz: hz,
			steps: steps,
			step_duration: 1000_000u64 / (hz as u64) / (steps as u64),
			led_power: led_power,
		}
	}

	pub fn start<F>(&self, on_update: F) -> std::sync::mpsc::SyncSender<Vec<f32>> where F: Fn(Vec<bool>) {
		let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<f32>>(0);
		let begin = std::time::Instant::now();
		let hz = self.hz as u128;
		let step_duration = self.step_duration;
		let mut led_power: Vec<f32> = Vec::new();

		std::thread::spawn(move|| {
			let mut led_state: Vec<bool> = Vec::new();
			loop {
				let mut has_changes = false;
				let position = (((begin.elapsed().as_micros() / hz) % 1_000_000u128) as f32) / 1_000_000f32;
				for (i,x) in led_power.iter().enumerate() {
					let new_value: bool = x >= &position;

					if new_value != led_state[i] {
						has_changes = true;
						led_state[i] = new_value;
						assert!(false);
					}
				}
				//(on_update)(led_state);
				{
					let result = rx.try_recv();
					match result {
						Ok(v) => led_power = v,
						Err(e) => match e {
							std::sync::mpsc::TryRecvError::Empty => {}, // OK!
							std::sync::mpsc::TryRecvError::Disconnected => {panic!("Got disconnected");} // XXX How to handle...?
						}
					};
				}
				std::thread::sleep(std::time::Duration::from_micros(step_duration));
			}
		});

		tx
	}

	pub fn set(&mut self, index: usize, mut power: f32) {
		if power > 1f32 { power = 1f32; } else if power < 0f32 { power = 0f32; }
		self.led_power[index] = power;
	}
}
