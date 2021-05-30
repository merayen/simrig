/** Lights up LEDs according to RPM and state (like when gear is in N, two blue LEDs) **/
use crate::led;

pub struct RPMLEDs<'a> {
	led_controller: &'a mut led::LEDController,
}

fn get_time() -> u128 {
	return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
}

impl<'a> RPMLEDs<'a> {
	pub fn new(led_controller: &'a mut led::LEDController) -> RPMLEDs<'a> {
		return RPMLEDs {
			led_controller: led_controller,
		}
	}

	pub fn honkey_donk_it(&mut self) {
		let mut a = 0;
		let mut next = get_time();

		loop {
			let time = get_time();
			if next < time {
				next = time + 100;
				a = (a + 1) % led::LED_MAX_POWER;
			}

			for i in 0..led::LED_COUNT {
				self.led_controller.set(i, a + i as u8);
			}

			unsafe {
				libc::usleep(2000);
			}

			self.led_controller.update(|led_index, value|{}); // TODO make lambda enable/disable LEDs
		}
	}
}
