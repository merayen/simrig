mod util;
mod led;
mod ui;
mod rpmleds;
mod pages;
mod fonts;
mod sources;

use crate::util::get_time;

// This method exists as I gave up fighting "ProjectCars2 does not have start()"-like error, though both the trait and itself has it...
fn shitty_hack(telemetrySource: &mut impl sources::SourceListener) -> std::sync::mpsc::Receiver<sources::Telemetry> {
	telemetrySource.start()
}

fn main() {
	let mut ctrl = led::LEDController::new();
	let mut rpmleds = rpmleds::RPMLEDs::new(&mut ctrl);
	let mut ui = ui::UI::new();
	let mut logo = pages::logo::Logo::new();
	let mut main = pages::main::Main::new();
	let mut test = pages::test::Test::new();
	let mut telemetrySource = sources::pc2::ProjectCars2::new();
	let telemetryChannel = shitty_hack(&mut telemetrySource);

	loop {
		let telemetry = telemetryChannel.try_recv();
		if telemetry.is_ok() {
			main.set_telemetry(telemetry.unwrap());
		}
		let t = get_time() as f64 / 5.0;
		main.rpm = (t % 5000.0) as f32 / 5000.0;
		main.brake = ((t + 333.0) % 1000.0) as f32 / 1000.0;
		main.clutch = ((t + 666.0) % 1000.0) as f32 / 1000.0;
		//main.gear = ((get_time() / 1000) % 8) as i8 - 1;

		for i in 0..4 {
			main.tire_wear[i] = ((((t) + 250.0 * i as f64) % 1000.0) / 1000.0) as f32;
			main.tire_temperature[i] = ((((t/10.0) + 250.0 * i as f64) % 1000.0) / 1000.0) as f32;
		}

		ui.draw(&mut main);

		unsafe {
			libc::usleep(1000000 / 30);
		}
	}
	//rpmleds.honkey_donk_it();
}
