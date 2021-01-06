mod util;
mod led;
mod ui;
mod rpmleds;
mod pages;
mod fonts;

use crate::util::get_time;

fn main() {
	let mut ctrl = led::LEDController::new();
	let mut rpmleds = rpmleds::RPMLEDs::new(&mut ctrl);
	let mut ui = ui::UI::new();
	let mut logo = pages::logo::Logo::new();
	let mut main = pages::main::Main::new();
	loop {
		let t = get_time() as f64 / 5.0;
		main.throttle = (t % 1000.0) as f32 / 1000.0;
		main.brake = ((t + 333.0) % 1000.0) as f32 / 1000.0;
		main.clutch = ((t + 666.0) % 1000.0) as f32 / 1000.0;
		main.gear = ((get_time() / 1000) % 8) as i8 - 1;
		ui.draw(&mut main);
		unsafe {
			libc::usleep(1000000 / 30);
		}
	}
	//rpmleds.honkey_donk_it();
}
