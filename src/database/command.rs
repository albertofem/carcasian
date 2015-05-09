use std::collections::HashMap;

pub trait Command {
	fn handle(mut database: HashMap<String, String>, command: &'static str) -> &'static str;
}