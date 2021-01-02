use std::io::prelude::*;

// Registers, 32-bit address offsets
const GPFSEL: isize = 0x00 / 4;
const GPFSEL0: isize = 0x00 / 4;
const GPFSEL1: isize = 0x04 / 4;
const GPSET0: isize = 0x1C / 4;
const GPCLR0: isize = 0x28 / 4;

// vec index is the LED No., and the content is the actual GPIO No.
const LED_GPIO: [u8; 10] = [17, 27, 22, 23, 24, 25, 5, 6, 16, 26];

// Value for each LED. 0 = no light. 127 = 50% light, 255 = yes
static led_values: [u8; 10] = [0; 10];

struct LEDController {
	position: u8,
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

	unsafe fn new() -> LEDController {
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


	unsafe fn update(&mut self) {
		self.position += 1;
		let mut gpsel = *self.gpio.offset(GPSET0);
		let mut gpclr = *self.gpio.offset(GPCLR0);

		// Create the mask
		//let mut mask: u32 = LED_GPIO.map( |x| 1<<*x).sum();

		for (i,x) in self.led_power.iter().enumerate() {
			assert!(*x < 30);
			if *x >= self.position {
				gpsel |= 1<<LED_GPIO[i];
			} else {
				gpclr |= 1<<LED_GPIO[i];
			}
		}
	}

	fn set(&mut self, led: usize, power: u8) {
		self.led_power[led] = power;
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

/// Set a LED on or off.
unsafe fn set_led(gpio: *mut u32, led: u8, value: bool) {
	assert!(led < LED_GPIO.len() as u8);
	let mut mask: u32 = !(0x7u32 << 21);
	let mut fsel = *gpio.offset(GPFSEL1);
	fsel &= mask; // Clear the 3 bits for the port
	fsel |= 0x1 << 21; // Set out flag that this should be an output port
	gpio.offset(GPFSEL1).write(fsel);

	if value {
		let mut val = *gpio.offset(GPSET0);
		val |= 1 << 17;
		gpio.offset(GPSET0).write(val);
	} else {
		let mut clr = *gpio.offset(GPCLR0);
		clr |= 1 << 17;
		gpio.offset(GPCLR0).write(clr);
	}
}

fn main() {

}
