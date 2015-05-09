use std::collections::HashMap;

use super::command::Command;

pub struct Set {
	name: &'static str
}

impl Command for Set {
	fn handle(mut database: &HashMap<String, String>,
			  command: &str,
			  arguments: Vec<&str>) -> &'static str {
		"(true)"
	}
}

impl Set {
	fn set(key: String, value: String) -> &'static str {
		"1"
	}

	fn get(key: String) -> &'static str {
		"1"
	}
}