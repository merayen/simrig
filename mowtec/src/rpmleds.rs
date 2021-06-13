/** Lights up LEDs according to RPM and state (like when gear is in N, two blue LEDs) **/
use crate::led;

pub struct RPMLEDs {
	start_rpm: u16,
	stop_rpm: u16,
	led_count: u8,
}

impl RPMLEDs {
	 /// start_rpm is when the first green LED on the left side is lit 100%
	 /// * `max_rpm` is when the meter maxes out (rightmost side)
	 /// * `led_count` yeah, guess what this is!
	 /// * `center_width` the width of the 100% lit LEDs
	 /// 
	pub fn new(start_rpm: u16, stop_rpm: u16, led_count: u8) -> RPMLEDs {
		assert!(start_rpm <= stop_rpm);

		return RPMLEDs {
			start_rpm: start_rpm,
			stop_rpm: stop_rpm,
			led_count: led_count,
		}
	}

	pub fn update(&self, mut rpm: u16) -> Vec<f32> {
		let mut result = Vec::new();

		if rpm > self.stop_rpm {
			rpm = self.stop_rpm;
		}

		let range = self.stop_rpm - self.start_rpm;
		let width = 0.01f32;
		let smoothness = 2f32;

		for i in 0..self.led_count {
			let v = (rpm as i32 - self.start_rpm as i32) as f32 / (self.stop_rpm - self.start_rpm) as f32;
			let divider = (i as f32 / self.led_count as f32 - v as f32).abs().powf(smoothness) / width;
			if divider >= 1f32 {
				result.push(1f32 / divider);
			} else {
				result.push(1f32);
			}
		}

		return result;
	}
}
