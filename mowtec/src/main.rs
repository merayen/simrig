fn main() {
	// https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_ptr
	let path = std::ffi::CString::new("/dev/gpiomem").unwrap();
	unsafe {
		let mem_fd = libc::open(path.as_ptr(), libc::O_RDWR | libc::O_SYNC | libc::O_CLOEXEC);
		if mem_fd < 0 {
			panic!("Could not open /dev/gpiomem");
		}

		let gpio_map = libc::mmap(
			std::ptr::null_mut(),
			4 * 1024,
			libc::PROT_READ | libc::PROT_WRITE,
			libc::MAP_SHARED,
			mem_fd,
			0x200000
			//(0xFE000000i64 + 0x200000i64) as i32
		);

		libc::close(mem_fd);

		if gpio_map == libc::MAP_FAILED {
			panic!("Could not mmap GPIO");
		}
	}
}
