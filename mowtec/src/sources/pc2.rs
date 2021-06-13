// Project Cars 2 data retriever
use crate::sources::{SourceListener, Telemetry};
use std::convert::TryInto;

pub struct ProjectCars2 { }

impl SourceListener for ProjectCars2 {
	fn start(&self) -> std::sync::mpsc::Receiver<Telemetry> {
		let (tx, rx) = std::sync::mpsc::sync_channel::<Telemetry>(0);
		std::thread::spawn(move || {
			let socket = std::net::UdpSocket::bind("192.168.1.255:5606").unwrap();
			socket.set_nonblocking(true).unwrap();

			let mut buf = [0; 560];
			let mut raw_telemetry = [0; 560];

			loop {

				// Gather the latest packet, we don't care to replay old ones
				for _ in 0..1000 { // Don't skip messages forever
					let read_results = socket.recv_from(&mut buf);
					if read_results.is_ok() {
						let (size, _) = read_results.unwrap();
						if size == 559 {
							raw_telemetry = buf;
						}
					} else {
						break; // No more data, we break
					}
				}

				let gear: i8;
				if raw_telemetry[45] == 111 {
					gear = -1;
				} else if raw_telemetry[45] >= 96 && raw_telemetry[45] < 96 + 10 {
					gear = (raw_telemetry[45] - 96) as i8;
				} else {
					gear = 10; // ???
				}

				let telemetry = Telemetry {
					throttle: raw_telemetry[13] as f32 / 255.0,
					brake: raw_telemetry[14] as f32 / 255.0,
					clutch: raw_telemetry[16] as f32 / 255.0,
					oil_temperature: (raw_telemetry[18] as i32 + ((raw_telemetry[19] as i32) << 8)) as f32, // TODO this correct?
					water_temperature: 0.0,
					fuel_level: f32::from_bits(u32::from_le_bytes(raw_telemetry[32..36].try_into().unwrap())),
					speed: 0.0,
					rpm: (raw_telemetry[40] as i32 + ((raw_telemetry[41] as i32) << 8)) as u16,
					rpm_max: (raw_telemetry[42] as i32 + ((raw_telemetry[43] as i32) << 8)) as u16,
					gear_count: 0,
					gear: gear,
					engine_damage: 0.0,
					engine_torque: 0.0,
				};
				tx.send(telemetry).unwrap();
			} 
		});

		rx
	}
}

impl ProjectCars2 {
	pub fn new() -> Self {
		return ProjectCars2 {};
	}
}
