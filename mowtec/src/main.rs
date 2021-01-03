mod util;
mod led;
mod ui;
mod rpmleds;
mod pages;
mod fonts;

fn main() {
	let mut ctrl = led::LEDController::new();
	let mut rpmleds = rpmleds::RPMLEDs::new(&mut ctrl);
	let mut ui = ui::UI::new();
	let mut logo = pages::logo::Logo::new();
	let mut main = pages::main::Main::new();
	loop {
		ui.draw(&mut main);
		unsafe {
			libc::usleep(1000000 / 30);
		}
	}
	//rpmleds.honkey_donk_it();
}
