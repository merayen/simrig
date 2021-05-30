use std::io::prelude::*;

pub const GPIO_COUNT: usize = 10;
pub const LED_MAX_POWER: u8 = 10;

// Registers, 32-bit address offsets
const GPSET0: isize = 0x1C / 4;
const GPCLR0: isize = 0x28 / 4;

// Which GPIO ports to initialize as outputs
const GPIO_IN: [u8; GPIO_COUNT] = [17, 27, 22, 23, 24, 25, 5, 6, 16, 26];

// Which GPIO ports to initialize as inputs
const GPIO_OUT: [u8; GPIO_COUNT] = [17, 27, 22, 23, 24, 25, 5, 6, 16, 26];

pub struct GPIO {
	gpio: *mut u32,
	gpio_map: *mut libc::c_void,
}

/**
 * Instance only once, or shame on you!
 *
 * Choir in the background: Shame! Shame! Shame!
 */
impl GPIO {
	fn check_platform() {
		let mut file = std::fs::File::open("/sys/firmware/devicetree/base/model").unwrap();
		let mut version = String::new();
		file.read_to_string(&mut version).unwrap();
		assert!(version.starts_with("Raspberry Pi 4 Model B Rev"), "I only do RPi 4B");
	}

	pub fn new() -> GPIO {
		GPIO::check_platform();

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
				panic!("This is not a BCM2711! This code is hardcoded for that exact chip, not something identifying as {}", *gpio.offset(60));
			}

			GPIO::configure(gpio);
		}

		GPIO {
			gpio: gpio,
			gpio_map: gpio_map,
		}
	}

	pub fn set(pin: u8, value: bool) {
		if value {
			let mut gpsel;
			unsafe {
				gpsel = *self.gpio.offset(GPSET0);
			}
			gpsel |= (1 << pin);
			unsafe {
				self.gpio.offset(GPSET0).write(gpsel);
			}
		} else {
			let mut gpclr;
			unsafe {
				gpclr = *self.gpio.offset(GPCLR0);
			}
			gpclr |= (1 << pin);
			unsafe {
				self.gpio.offset(GPCLR0).write(gpclr);
			}
		}
	}

	// Sets the FSEL... values for all the LEDs
	unsafe fn configure(gpio: *mut u32) {
		let mut mask = [0u32; 3];
		let mut val = [0u32; 3];

		// Sanity check
		for a in GPIO_OUT.iter() {
			for b in GPIO_IN.iter() {
				assert!(a != b);
			}
		}

		fn setPinMode(gpio_pin: u32, value: u32) {
			assert!(*gpio_pin < 30); // RPi doesn't support any more anyway for the GPIO port
			assert!(value <= 0x7u32);

			let register_index: usize = (gpio_pin / 10).into();

			// Calculate the mask we will clear with
			mask[register_index] |= 0x7u32 << ((gpio_pin % 10) * 3);

			// Store the value
			val[register_index] |= value << ((gpio_pin % 10) * 3);
		}

		// Configure output pins
		for gpio_pin in GPIO_OUT.iter() {
			setPinMode(gpio_pin, 0x1u32);
		}

		// Configure input pins
		for gpio_pin in GPIO_IN.iter() {
			setPinMode(gpio_pin, 0x0u32);
		}

		for register_index in 0..mask.len() { // register_index represents 32 bit offset in register
			let mut fsel_value = *gpio.offset(register_index as isize);
			//println!("{}, {}, {}", register_index, !mask[register_index], val[register_index]);
			//let before = fsel_value;
			fsel_value &= !mask[register_index]; // Clears the bits we are supposed to modify first
			fsel_value |= val[register_index];
			//println!("mask:        {:032b}", !mask[register_index]);
			//println!("val:         {:032b}", val[register_index]);
			//println!("fsel before: {:032b}", before);
			//println!("fsel after:  {:032b}", fsel_value);
			gpio.offset(register_index as isize).write(fsel_value);
		}
	}
}

impl Drop for GPIO {
	fn drop(&mut self) {
		unsafe {
			if libc::munmap(self.gpio_map, 4 * 1024) != 0 {
				panic!("Could not munmap");
			}
		}
	}
}

