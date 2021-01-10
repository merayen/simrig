pub mod pc2;

#[derive(Debug)]
pub struct Telemetry {
	throttle: f32,
	brake: f32,
	clutch: f32,
	oil_temperature: f32,
	water_temperature: f32,
	fuel_level: f32,
	speed: f32,
	rpm: u16,
	rpmMax: u16,
	gear_count: u8,
	gear: i8, // TODO
	engine_damage: f32,
	engine_torque: f32,
}

trait SourceListener {
	fn start(&self) -> std::sync::mpsc::Receiver<Telemetry>;
}

