pub mod pc2;

#[derive(Debug)]
pub struct Telemetry {
	pub throttle: f32,
	pub brake: f32,
	pub clutch: f32,
	pub oil_temperature: f32,
	pub water_temperature: f32,
	pub fuel_level: f32,
	pub speed: f32,
	pub rpm: u16,
	pub rpm_max: u16,
	pub gear_count: u8,
	pub gear: i8, // TODO
	pub engine_damage: f32,
	pub engine_torque: f32,
}

pub trait SourceListener {
	fn start(&self) -> std::sync::mpsc::Receiver<Telemetry>;
}

pub fn blank_telemetry() -> Telemetry {
	Telemetry {
		throttle: 0.0,
		brake: 0.0,
		clutch: 0.0,
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
	}
}

