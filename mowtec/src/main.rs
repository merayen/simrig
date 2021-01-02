use std::io::prelude::*;

// Registers, 32-bit address offsets
const GPFSEL: isize = 0x00 / 4;
const GPFSEL0: isize = 0x00 / 4;
const GPFSEL1: isize = 0x04 / 4;
const GPSET0: isize = 0x1C / 4;
const GPCLR0: isize = 0x28 / 4;

// vec index is the LED No., and the content is the actual GPIO No.
const LED_GPIO: [u8; 10] = [17, 27, 22, 23, 24, 25, 5, 6, 16, 26];

struct LEDController {
	position: u8,

	// Value for each LED. 0 = no light. 127 = 50% light, 255 = yes
	led_power: [u8; LED_GPIO.len()],

	gpio: *mut u32,
	gpio_map: *mut libc::c_void,
}

impl LEDController {
	fn check_platform() {
		let mut file = std::fs::File::open("/sys/firmware/devicetree/base/model").unwrap();
		let mut version = String::new();
		file.read_to_string(&mut version).unwrap();
		assert!(version.starts_with("Raspberry Pi 4 Model B Rev"));
	}

	fn new() -> LEDController {
		LEDController::check_platform();

		// https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_ptr
		let path = std::ffi::CString::new("/dev/gpiomem").unwrap();

		let &gpio_map;
		let &mem_fd;

		unsafe {
			mem_fd = libc::open(path.as_ptr(), libc::O_RDWR | libc::O_SYNC | libc::O_CLOEXEC);
		}

		if mem_fd < 0 {
			panic!("Could not open /dev/gpiomem");
		}

		unsafe {
			gpio_map = libc::mmap(
				std::ptr::null_mut(),
				4 * 1024,
				libc::PROT_READ | libc::PROT_WRITE,
				libc::MAP_SHARED,
				mem_fd,
				0
			);

			libc::close(mem_fd);
		}

		if gpio_map == libc::MAP_FAILED {
			panic!("Could not mmap GPIO");
		}

		// Direct mapping of GPIO registers that we can read and write to
		let mut gpio = gpio_map as *mut u32;

		unsafe {
			// Make additional checks the IC is the BCM2711 (?)
			if *gpio.offset(60) == 0x6770696f {
				panic!("This is not a BCM2711! This code is hardcoded for that exact chip {}", *gpio.offset(60));
			}

			LEDController::prep_leds(gpio);
		}

		LEDController {
			position: 0,
			led_power: [0u8; 10],
			gpio: gpio,
			gpio_map: gpio_map,
		}
	}


	fn update(&mut self) {
		self.position += 1;
		self.position %= 100;

		let mut gpsel;
		let mut gpclr;

		unsafe {
			gpsel = *self.gpio.offset(GPSET0);
			gpclr = *self.gpio.offset(GPCLR0);
		}

		for (i,x) in self.led_power.iter().enumerate() {
			let v = 1<<LED_GPIO[i];
			if *x >= self.position {
				gpsel |= v;
			} else {
				gpclr |= v;
			}
		}

		self.gpio.offset(GPSET0).write(gpsel);
		self.gpio.offset(GPCLR0).write(gpclr);
	}

	fn set(&mut self, led: usize, power: u8) {
		if power > 100 {
			self.led_power[led] = 100;
		} else {
			self.led_power[led] = power;
		}
	}

	// Sets the FSEL... values for all the LEDs
	unsafe fn prep_leds(gpio: *mut u32) {
		let mut mask = [0u32; 3];
		let mut val = [0u32; 3];

		for (led, gpio_pin) in LED_GPIO.iter().enumerate() {
			assert!(*gpio_pin < 30); // RPi doesn't support any more anyway for the GPIO port

			let registerIndex: usize = (gpio_pin / 10).into();

			// Calculate the mask
			mask[registerIndex] |= 0x7u32 << ((gpio_pin % 10) * 3);

			// Then do the FSEL value
			val[registerIndex] |= 0x1u32 << ((gpio_pin % 10) * 3);
		}

		for registerIndex in 0..mask.len() { // registerIndex represents 32 bit offset in register
			let mut fselValue = *gpio.offset(registerIndex as isize);
			println!("{}, {}, {}", registerIndex, !mask[registerIndex], val[registerIndex]);
			let before = fselValue;
			fselValue &= !mask[registerIndex];
			fselValue |= val[registerIndex];
			println!("mask:        {:032b}", !mask[registerIndex]);
			println!("val:         {:032b}", val[registerIndex]);
			println!("fsel before: {:032b}", before);
			println!("fsel after:  {:032b}", fselValue);
			gpio.offset(registerIndex as isize).write(fselValue);
		}
	}
}

impl Drop for LEDController {
	fn drop(&mut self) {
		unsafe {
			if libc::munmap(self.gpio_map, 4 * 1024) != 0 {
				panic!("Could not munmap");
			}
		}
	}
}

fn main() {
	let mut ctrl = LEDController::new();
	loop {
		ctrl.set(0, 50);
		unsafe {
			libc::usleep(1000);
		}
		ctrl.update();
	}
}
