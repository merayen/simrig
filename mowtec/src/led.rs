use std::io::prelude::*;

pub const LED_COUNT: usize = 10;
pub const LED_MAX_POWER: u8 = 10;

// Registers, 32-bit address offsets
const GPSET0: isize = 0x1C / 4;
const GPCLR0: isize = 0x28 / 4;

// vec index is the LED No., and the content is the actual GPIO No.
const LED_GPIO: [u8; LED_COUNT] = [17, 27, 22, 23, 24, 25, 5, 6, 16, 26];

pub struct LEDController {
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

	pub fn new() -> LEDController {
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
		let gpio = gpio_map as *mut u32;

		unsafe {
			// Make additional checks the IC is the BCM2711 (?)
			if *gpio.offset(60) == 0x6770696f {
				panic!("This is not a BCM2711! This code is hardcoded for that exact chip {}", *gpio.offset(60));
			}

			LEDController::prep_leds(gpio);
		}

		LEDController {
			position: 0,
			led_power: [0u8; LED_COUNT],
			gpio: gpio,
			gpio_map: gpio_map,
		}
	}


	pub fn update(&mut self) {
		self.position += 1;
		self.position %= LED_MAX_POWER + 1;

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

		unsafe {
			self.gpio.offset(GPSET0).write(gpsel);
			self.gpio.offset(GPCLR0).write(gpclr);
		}
	}

	pub fn set(&mut self, led: usize, power: u8) {
		if power > LED_MAX_POWER {
			self.led_power[led] = LED_MAX_POWER;
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

			let register_index: usize = (gpio_pin / 10).into();

			// Calculate the mask
			mask[register_index] |= 0x7u32 << ((gpio_pin % 10) * 3);

			// Then do the FSEL value
			val[register_index] |= 0x1u32 << ((gpio_pin % 10) * 3);
		}

		for register_index in 0..mask.len() { // register_index represents 32 bit offset in register
			let mut fsel_value = *gpio.offset(register_index as isize);
			//println!("{}, {}, {}", register_index, !mask[register_index], val[register_index]);
			//let before = fsel_value;
			fsel_value &= !mask[register_index];
			fsel_value |= val[register_index];
			//println!("mask:        {:032b}", !mask[register_index]);
			//println!("val:         {:032b}", val[register_index]);
			//println!("fsel before: {:032b}", before);
			//println!("fsel after:  {:032b}", fsel_value);
			gpio.offset(register_index as isize).write(fsel_value);
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

