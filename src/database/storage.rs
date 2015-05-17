use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn new() -> Arc<Mutex<HashMap<String, String>>>
{
	Arc::new(Mutex::new(HashMap::new()))
}