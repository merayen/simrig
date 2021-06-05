/** Driver for the IC */
use std::io::prelude::*;

pub struct MCP23S17<F> {
	addr: u8, // 0 to 7, which address this IC is
	output_ports: u16,
	fd: libc::c_int,
	spi: spidev::Spidev,
	cs_pin_func: F,
}

pub const IODIRA: u8 = 0x00;
pub const IODIRB: u8 = 0x01;
pub const IPOLA: u8 = 0x02;
pub const IPOLB: u8 = 0x03;
pub const GPINTENA: u8 = 0x04;
pub const GPINTENB: u8 = 0x05;
pub const DEFVALA: u8 = 0x06;
pub const DEFVALB: u8 = 0x07;
pub const INTCONA: u8 = 0x08;
pub const INTCONB: u8 = 0x09;
pub const IOCON: u8 = 0x0A;
pub const GPPUA: u8 = 0x0C;
pub const GPPUB: u8 = 0x0D;
pub const INTFA: u8 = 0x0E;
pub const INTFB: u8 = 0x0F;
pub const INTCAPA: u8 = 0x10;
pub const INTCAPB: u8 = 0x11;
pub const GPIOA: u8 = 0x12;
pub const GPIOB: u8 = 0x13;
pub const OLATA: u8 = 0x14;
pub const OLATB: u8 = 0x15;

impl<F> MCP23S17<F> where F: Fn(bool) {
	pub fn new(device: &str, addr: u8, output_ports: u16, cs_pin_func: F) -> MCP23S17<F> {
		assert!(addr < 8);
		let mut instance = MCP23S17 {
			addr: addr,
			output_ports: output_ports,
			fd: 0,
			spi: spidev::Spidev::open(device).unwrap(), // TODO Probably move out so we can use the SPI pins for other ICs too!
			cs_pin_func: cs_pin_func,
		};

		MCP23S17::configure(&mut instance, device);

		instance
	}

	fn configure(instance: &mut MCP23S17<F>, device: &str) {
		((*instance).cs_pin_func)(true); // CS is high when chip is not selected. Make sure it starts there

		let options = spidev::SpidevOptions::new()
			.bits_per_word(8)
			.max_speed_hz(1_000)
			.mode(spidev::SpiModeFlags::SPI_MODE_0)
			.build();

		assert!((*instance).spi.configure(&options).is_ok());

		// Init output ports
		MCP23S17::send(instance, IODIRA, 0u8);
		MCP23S17::send(instance, IODIRB, 0u8);
		//MCP23S17::send(instance, IODIRA, (!(*instance).output_ports & 255) as u8);
		//MCP23S17::send(instance, IODIRB, (!(*instance).output_ports >> 8) as u8);
		MCP23S17::send(instance, GPIOA, 255u8);
		MCP23S17::send(instance, GPIOB, 255u8);
		MCP23S17::send(instance, OLATA, 255u8);
		MCP23S17::send(instance, OLATB, 255u8);

		assert!(false, "Det virker!");
	}

	fn send(&mut self, register: u8, value: u8) {
		(self.cs_pin_func)(false);
		self.spi.write(&[self.addr | 64, register, value]).unwrap();
		(self.cs_pin_func)(true);
	}

	pub fn end(&self) {
		unsafe {
			libc::close(self.fd);
		}
	}
}
