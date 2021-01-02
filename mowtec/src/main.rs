mod led;
mod ui;

fn get_time() -> u128 {
	return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
}

fn main() {
	let mut ctrl = led::LEDController::new();
	let mut a = 0;
	let mut next = get_time();
	let mut ui = ui::UI::new();

	loop {
		let time = get_time();
		if next < time {
			next = time + 100;
			a = (a + 1) % led::LED_MAX_POWER;
		}

		for i in 0..led::LED_COUNT {
			ctrl.set(i, a + i as u8);
		}

		unsafe {
			libc::usleep(2000);
		}

		ctrl.update();
	}
}
