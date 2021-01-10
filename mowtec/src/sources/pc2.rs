// Project Cars 2 data retriever
use crate::sources::{SourceListener, Telemetry};

pub struct ProjectCars2 {
	lol: u8,
}

impl SourceListener for ProjectCars2 {
	fn start(&self) -> std::sync::mpsc::Receiver<Telemetry> {
		let (tx, rx) = std::sync::mpsc::sync_channel::<Telemetry>(0);
		let mut thread = std::thread::spawn(move || {

			let mut socket = std::net::UdpSocket::bind("192.168.1.255:5606").unwrap();
			socket.set_nonblocking(true);

			let mut buf = [0; 560];
			let mut rawTelemetry = [0; 560];

			loop {

				// Gather the latest packet, we don't care to replay old ones
				loop {
					let read_results = socket.recv_from(&mut buf);
					if read_results.is_ok() {
						let (size, source) = read_results.unwrap();
						if size == 559 {
							rawTelemetry = buf;
						}
					} else {
						break; // No more data, we break
					}
				}

				let telemetry = Telemetry {
					throttle: rawTelemetry[13] as f32 / 255.0,
					brake: rawTelemetry[14] as f32 / 255.0,
					clutch: rawTelemetry[16] as f32 / 255.0,
					oil_temperature: 0.0,
					water_temperature: 0.0,
					fuel_level: 0.0,
					speed: 0.0,
					rpm: 0,
					rpm_max: 0,
					gear_count: 0,
					gear: 0, // TODO
					engine_damage: 0.0,
					engine_torque: 0.0,
				};
				tx.send(telemetry);
			} 
		});

		rx
	}
}

impl ProjectCars2 {
	pub fn new() -> Self {
		return ProjectCars2 {lol: 0};
	}
}
