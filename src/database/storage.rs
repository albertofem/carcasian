use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Storage<'a> {
    data: Arc<Mutex<HashMap<&'a str, &'a str>>>,
}

impl <'a> Storage<'a> {
    pub fn new() -> Result<Storage<'a>, u8> {
        let storage = Storage {
            data: Arc::new(Mutex::new(HashMap::new()))
        };

        Ok(storage)
    }

    pub fn get(&mut self, key: &'a str) -> Result<&'a str, u8> {
        let data = self.data.clone();
        let locked_data = data.lock().unwrap();

        if !locked_data.contains_key(key) {
            return Err(1)
        }

        Ok(locked_data.get(key).unwrap())
    }

    pub fn set(&mut self, key: &'a str, value: &'a str) -> Result<bool, u8> {
        let data = self.data.clone();
        let mut locked_data = data.lock().unwrap();

        locked_data.insert(key, value);

        Ok(true)
    }

    pub fn del(&mut self, key: &'a str) -> Result<bool, u8> {
        let data = self.data.clone();
        let mut locked_data = data.lock().unwrap(); // lock will be free when out of scope

        if !locked_data.contains_key(key) {
            return Err(1)
        }

        locked_data.remove(key);

        Ok(true)
    }

    pub fn count_keys(&self) -> usize {
        let data = self.data.clone();
        let locked_data = data.lock().unwrap();

        locked_data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn test_database_set_get() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test", "test").unwrap(), true);
        assert_eq!(storage.get("test").unwrap().to_string(), "test".to_string());
    }

    #[test]
    fn test_get_with_no_data() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.get("test").is_err(), true);
    }

    #[test]
    fn test_set_overwrite_key() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test", "test").unwrap(), true);
        assert_eq!(storage.get("test").unwrap().to_string(), "test".to_string());
        assert_eq!(storage.set("test", "test2").unwrap(), true);
        assert_eq!(storage.get("test").unwrap().to_string(), "test2".to_string());
        assert_eq!(storage.count_keys(), 1);
    }

    #[test]
    fn test_delete_key() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test", "test").unwrap(), true);
        assert_eq!(storage.get("test").unwrap().to_string(), "test".to_string());
        assert_eq!(storage.del("test").unwrap(), true);
        assert_eq!(storage.get("test").is_err(), true);
        assert_eq!(storage.count_keys(), 0);
    }

    #[test]
    fn test_count_keys() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test", "test").unwrap(), true);
        assert_eq!(storage.set("test2", "test2").unwrap(), true);

        assert_eq!(storage.count_keys(), 2);
    }
}