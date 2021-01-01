fn get_address(gpio_pin_no: u8) -> i32 {
	if gpio_pin_no > 27 {
		panic!("Pin number {} does not exist", gpio_pin_no);
	}

	return 0;
}

fn main() {
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

	unsafe {
		let mut stat: libc::stat = std::mem::zeroed();
		libc::fstat(mem_fd, &mut stat);
		//println!("{}", stat.st_size);
	}

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
