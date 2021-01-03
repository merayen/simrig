mod util;
mod led;
mod ui;
mod rpmleds;
mod pages;

fn main() {
	let mut ctrl = led::LEDController::new();
	let mut rpmleds = rpmleds::RPMLEDs::new(&mut ctrl);
	let mut ui = ui::UI::new();
	let mut logo = pages::logo::Logo::new();
	loop {
		ui.draw(&mut logo);
		unsafe {
			libc::usleep(1000 / 30);
		}
	}
	//rpmleds.honkey_donk_it();
}
