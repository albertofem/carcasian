use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Result<Storage, u8> {
        let storage = Storage {
            data: HashMap::new()
        };

        Ok(storage)
    }

    pub fn get(&mut self, key: String) -> Result<&String, u8> {
        if !self.data.contains_key(&key) {
            return Err(1)
        }

        Ok(self.data.get(&key).unwrap())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool, u8> {
        self.data.insert(key, value);

        Ok(true)
    }

    pub fn del(&mut self, key: String) -> Result<bool, u8> {
        if !self.data.contains_key(&key) {
            return Err(1)
        }

        self.data.remove(&key);

        Ok(true)
    }

    pub fn count_keys(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn test_database_set_get() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
    }

    #[test]
    fn test_get_with_no_data() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.get("test".to_string()).is_err(), true);
    }

    #[test]
    fn test_set_overwrite_key() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
        assert_eq!(storage.set("test".to_string(), "test2".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test2".to_string());
        assert_eq!(storage.count_keys(), 1);
    }

    #[test]
    fn test_delete_key() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
        assert_eq!(storage.del("test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).is_err(), true);
        assert_eq!(storage.count_keys(), 0);
    }

    #[test]
    fn test_count_keys() {
        let mut storage = Storage::new().unwrap();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.set("test2".to_string(), "test2".to_string()).unwrap(), true);

        assert_eq!(storage.count_keys(), 2);
    }
}