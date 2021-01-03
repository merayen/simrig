pub fn get_time() -> u128 {
	return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
}
