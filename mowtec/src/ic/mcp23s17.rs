/** Driver for the IC */
use std::io::prelude::*;

pub struct MCP23S17 {
	output_ports: Vec<u8>,
}

impl MCP23S17 {
	pub fn new<F>(device: &str, output_ports: Vec<u8>, cs_pin_func: F) -> MCP23S17 where F: Fn(bool) {
		assert!(output_ports.iter().all(|x|*x<16));
		let instance = MCP23S17 {
			output_ports: output_ports,
		};

		MCP23S17::configure(&instance, cs_pin_func);

		instance
	}

	fn configure<F>(instance: &MCP23S17, cs_pin_func: F) where F: Fn(bool) {
		// TODO write config to IODIRA/B
		MCP23S17::write(instance, cs_pin_func);
	}

	fn write<F>(&self, cs_pin_func: F) where F: Fn(bool) {
		cs_pin_func(false);

		cs_pin_func(true);
	}
}
