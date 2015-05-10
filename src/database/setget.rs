use std::collections::HashMap;

pub struct SetGet;

impl SetGet
{
	pub fn set<'a>(&self, database: &'a mut HashMap<String, String>,
		   key: &str, value: &str) -> String
	{
			database.insert(key.to_string(), value.to_string());

			return "(true)".to_string();
	}

	pub fn get<'a>(&self, database: &'a mut HashMap<String, String>,
		   key: &str) -> &'a String
	{
		let data = database.get(key);

		data.unwrap()
	}
}