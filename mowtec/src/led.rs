/** Turns LEDs on and off. Also pulsewidth modulates them **/
use std::sync::mpsc::{SyncSender, Receiver};

pub struct LEDController {
	hz: u32,
	step_duration: u64, // Microseconds
}

impl LEDController {
	pub fn new(resolution: u8, hz: u32) -> LEDController {
		LEDController {
			hz: hz,
			step_duration: 1000_000u64 / (hz as u64) / (resolution as u64),
		}
	}

	pub fn start(&self) -> (SyncSender<Vec<f32>>, Receiver<Vec<bool>>) {
		let (tx_led_power, rx_led_power) = std::sync::mpsc::sync_channel::<Vec<f32>>(0); // We receive the PWM with for the LEDs
		let (tx_led_state, rx_led_state) = std::sync::mpsc::sync_channel::<Vec<bool>>(0); // We send out the LED state, the caller forwards this to GPIO

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

				if has_changes {
					tx_led_state.send(led_state.clone()).unwrap();
				}

				{
					let result = rx_led_power.try_recv();
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

		(tx_led_power, rx_led_state)
	}
}
