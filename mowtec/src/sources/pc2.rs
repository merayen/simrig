// Project Cars 2 data retriever
use crate::sources::{SourceListener, Telemetry};

struct ProjectCars2 {}

impl SourceListener for ProjectCars2 {
	fn start(&self) -> std::sync::mpsc::Receiver<Telemetry> {
		let (tx, rx) = std::sync::mpsc::sync_channel::<Telemetry>(0);
		let mut thread = std::thread::spawn(move || {

			let mut socket = std::net::UdpSocket::bind("192.168.1.255:5606").unwrap();

			let mut buf = [0; 560];


			loop {
				let (size, src) = socket.recv_from(&mut buf).unwrap();
				if size == 559 {
					let telemetry = Telemetry {
						throttle: buf[13] as f32 / 255.0,
						brake: buf[14] as f32 / 255.0,
						clutch: buf[16] as f32 / 255.0,
						oil_temperature: 0.0,
						water_temperature: 0.0,
						fuel_level: 0.0,
						speed: 0.0,
						rpm: 0,
						rpmMax: 0,
						gear_count: 0,
						gear: 0, // TODO
						engine_damage: 0.0,
						engine_torque: 0.0,
					};
					tx.send(telemetry);
				}
			} 
		});

		rx
	}
}

impl ProjectCars2 {
	pub fn new() -> ProjectCars2 {
		ProjectCars2 {}
	}
}
