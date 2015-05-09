use std::collections::HashMap;

use super::command::Command;

struct Set {
	name: &'static str
}

impl Command for Set {
	fn handle(mut database: HashMap<String, String>, command: &'static str) -> &'static str {
		"1"
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