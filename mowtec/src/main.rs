mod util;
mod led;
mod ui;
mod rpmleds;
mod pages;
mod fonts;
mod sources;
mod ic;

use crate::util::get_time;

// This method exists as I gave up fighting "ProjectCars2 does not have start()"-like error, though both the trait and itself has it...
fn shitty_hack(telemetry_source: &mut impl sources::SourceListener) -> std::sync::mpsc::Receiver<sources::Telemetry> {
	telemetry_source.start()
}

fn main() {
	let input_pins: Vec<u8> = Vec::new();
	let mut output_pins: Vec<u8> = Vec::new();
	output_pins.push(17); // Configure this GPIO 17 pin to be used for chip enable (CS) on the MCP23S17 chip for the LEDs
	output_pins.push(27); // RESET for all ICs connected (hopefully they have 0v reset)

	let (gpio_tx, gpio_rx) = std::sync::mpsc::sync_channel::<u16>(0); // Use gpio_tx to change GPIO pins on the MCP23S17

	let led = led::LEDController::new(5, 50);
	let (tx_led_power, rx_led_state) = led.start();

	// LEDController to gpio-ports
	std::thread::spawn(move||{
		loop {
			let led_state = rx_led_state.recv().unwrap();
			gpio_tx.send(led_state.iter().enumerate().map(|(i,x)|(*x as u16) << i).sum::<u16>()).unwrap();
		}
	});

	// Hardware communication thread
	std::thread::spawn(move||{
		let gpio = ic::gpio::GPIO::new(input_pins, output_pins); // TODO this is the problematic one

		// Reset all ICs connected via SPI
		let wait_time = std::time::Duration::from_millis(100);
		gpio.set(27, true); // Make it high, does not reset
		std::thread::sleep(wait_time);
		gpio.set(27, false); // Do the reset
		std::thread::sleep(wait_time);
		gpio.set(27, true); // Done resetting
		std::thread::sleep(wait_time);

		let mut mcp23s17 = ic::mcp23s17::MCP23S17::new("/dev/spidev0.0", 0u8, 65535, |value|{gpio.set(17, value)});
		loop {
			let pins = gpio_rx.recv();
			if pins.is_ok() {
				let noe = pins.unwrap();
				mcp23s17.set(noe);
			}
		}
	});

	let mut rpmleds = rpmleds::RPMLEDs::new(6000, 8000, 11);
	let ui = ui::UI::new();
	pages::logo::Logo::new();
	let mut main = pages::main::Main::new();
	pages::test::Test::new();
	let mut telemetry_source = sources::pc2::ProjectCars2::new();
	let telemetry_channel = shitty_hack(&mut telemetry_source);

	let mut rpm = 0u16;
	loop {
		let telemetry = telemetry_channel.try_recv();
		match telemetry {
			Ok(v) => {
				main.set_telemetry(v);
			}
			Err(e) => match e {
				std::sync::mpsc::TryRecvError::Empty => {}, // OK!
				std::sync::mpsc::TryRecvError::Disconnected => {panic!("Got disconnected");} // XXX How to handle...?
			}
		}

		let t = get_time() as f64 / 5.0;
		main.rpm = (t % 5000.0) as f32 / 5000.0;
		main.brake = ((t + 333.0) % 1000.0) as f32 / 1000.0;
		main.clutch = ((t + 666.0) % 1000.0) as f32 / 1000.0;
		//main.gear = ((get_time() / 1000) % 8) as i8 - 1;

		//for i in 0..4 {
			//main.tire_wear[i] = ((((t) + 250.0 * i as f64) % 1000.0) / 1000.0) as f32;
			//main.tire_temperature[i] = ((((t/10.0) + 250.0 * i as f64) % 1000.0) / 1000.0) as f32;
		//}

		ui.draw(&mut main);

		rpm += 100;
		rpm %= 9000;

		let led_brightnesses: Vec<f32> = rpmleds.update(rpm);

		//print!("{}: ", rpm);
		//for x in led_brightnesses.clone() {
		//	print!("{:.2} ", x);
		//}
		//println!("");

		tx_led_power.send(led_brightnesses).unwrap();

		unsafe { // thread sleep instead?
			libc::usleep(1000000 / 30);
		}
	}
}
