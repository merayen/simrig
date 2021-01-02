use std::io::prelude::*;


// Registers, 32-bit address offsets
const GPFSEL1: isize = 0x04 / 4;
const GPSET0: isize = 0x1C / 4;
const GPCLR0: isize = 0x28 / 4;

fn get_address(gpio_pin_no: u8) -> i32 {
	if gpio_pin_no > 27 {
		panic!("Pin number {} does not exist", gpio_pin_no);
	}

	return 0;
}

fn check_platform() {
	let mut file = std::fs::File::open("/sys/firmware/devicetree/base/model").unwrap();
	let mut version = String::new();
	file.read_to_string(&mut version).unwrap();
	assert!(version.starts_with("Raspberry Pi 4 Model B Rev"));
}

unsafe fn set_pin(gpio: *mut u32, pin: u8, value: bool) {
	//println!("Before: {}", *gpio.offset(GPFSEL1));
	//gpio.offset(GPFSEL1).write(*gpio.offset(GPFSEL1) & (0xFFFFFFFF ^ ((1<<21) + (1<<22) + (1<<23))) | (1<<21));
	//println!("After: {}", *gpio.offset(GPFSEL1));
	//gpio.offset(GPSET0).write(*gpio.offset(GPSET0) | 1<<17);
	let mut mask: u32 = !(0x7u32 << 21);
	let mut fsel = *gpio.offset(GPFSEL1);
	fsel &= mask; // Clear the 3 bits for the port
	fsel |= 0x1 << 21; // Set out flag that this should be an output port
	println!("FSEL before: {}", *gpio.offset(1));
	gpio.offset(GPFSEL1).write(fsel);
	println!("FSEL after: {}", *gpio.offset(1));

	if value {
		let mut val = *gpio.offset(GPSET0);
		println!("VAL before: {}", val);
		val |= (1 << 17);
		gpio.offset(GPSET0).write(val);
		println!("VAL before: {}", val);
		//gpio.offset(GPSET0).write(
	} else {
		let mut clr = *gpio.offset(GPCLR0);
		println!("CLR before: {}", clr);
		clr |= (1 << 17);
		gpio.offset(GPCLR0).write(clr);
		println!("CLR after: {}", clr);
	}
}

fn main() {
	check_platform();
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
			//0x200000
			//(0xFE000000i64 + 0x200000i64) as i32
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

		set_pin(gpio, 17, true);
	}

	// Configure 17 as output

	//unsafe {
	//	let mut stat: libc::stat = std::mem::zeroed();
	//	libc::fstat(mem_fd, &mut stat);
	//	println!("{}", stat.st_size);
	//}

	//let mut noe = gpio_map as *mut u8;

	//unsafe {
	//	for i in 0..1000 {
	//		println!("{}, {}", i, *noe.offset(i) as char);
	//	}
	//}

	unsafe {
		if libc::munmap(gpio_map, 4 * 1024) != 0 {
			panic!("Could not munmap");
		}
	}
}
