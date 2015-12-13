use std::collections::{HashMap, HashSet};
use std::fmt;

// This is an Enum contain basic data definition
// For now there is only plain values and sets
// All data are Strings in order to simplify
enum StorageValue {
    Plain(String),
    Set(HashSet<String>)
}

pub enum StorageError {
    KeyNotFound,
    InternalError
}

impl fmt::Debug for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            StorageError::KeyNotFound => "Key not found",
            StorageError::InternalError => "Internal error"
        };

        write!(f, "{}", message)
    }
}

// This is the main data structure we will be using
pub struct Storage {
    data: HashMap<String, StorageValue>,
}

// Let's implement Storage by adding all Redis
// commands we want to support
impl Storage {

    // This is a static method (not attached to an instance)
    // used to create an instance of the structure. It's idiomatic
    // to call these kind of methods as "new" in Rust
    pub fn new() -> Storage {
        Storage { data: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Result<&String, StorageError> {
        if !self.data.contains_key(&key) {
            return Err(StorageError::KeyNotFound)
        }

        let data = self.data.get(&key).unwrap();

        // We can use '*' to dereference because unwrap()
        // returns a reference to the data being retrieved
        // and we can match more easier by dereferencing
        match *data {
            StorageValue::Plain(ref d) => Ok(&d), // we return a reference to the data wrapped in a Result
            _ => Err(StorageError::InternalError)
        }
    }

    // As we are going to modify the HashMap (inserting)
    // we need to get a reference to self '&mut self'
    pub fn set(&mut self, key: String, value: String) -> Result<bool, StorageError> {
        // Here we instantiate a new enum value from the value
        // passed by the user
        let value = StorageValue::Plain(value);

        self.data.insert(key, value);

        Ok(true)
    }

    pub fn del(&mut self, key: String) -> Result<bool, StorageError> {
        if !self.data.contains_key(&key) {
            return Err(StorageError::KeyNotFound)
        }

        self.data.remove(&key);

        Ok(true)
    }

    pub fn sismember(&mut self, key: String, member: String) -> Result<bool, StorageError> {
        if !self.data.contains_key(&key) {
            return Err(StorageError::KeyNotFound)
        }

        let data = self.data.get(&key).unwrap();

        match *data {
            StorageValue::Set(ref set) => Ok(set.contains(&member)),
            _ => Err(StorageError::InternalError)
        }
    }

    pub fn smembers(&mut self, key: String) -> Result<&HashSet<String>, StorageError> {
        if !self.data.contains_key(&key) {
            return Err(StorageError::KeyNotFound)
        }

        let data = self.data.get(&key).unwrap();

        match *data {
            StorageValue::Set(ref d) => Ok(&d),
            _ => Err(StorageError::InternalError)
        }
    }

    pub fn srem(&mut self, key: String, member: String) -> Result<bool, StorageError> {
        if !self.data.contains_key(&key) {
            return Err(StorageError::KeyNotFound);
        }

        // We need to get a mutable reference to the data
        // because we are going to remove an element from the
        // set
        let data = self.data.get_mut(&key).unwrap();

        match *data {
            StorageValue::Set(ref mut d) => Ok(d.remove(&member)),
            _ =>Err(StorageError::InternalError)
        }
    }

    pub fn sadd(&mut self, key: String, member: String) -> Result<bool, StorageError> {
        if !self.data.contains_key(&key) {
            // Here, as HashMap is going to take ownership of "member"
            // we can just clone and pass a copy, as we need to use the
            // key just a little later. Made for illustration
            self.data.insert(key.clone(), StorageValue::Set(HashSet::new()));
        }

        // This reference to &key here wouldn't work if we
        // didn't passed a clone before to the insert method
        // in the HashMap.
        let data = self.data.get_mut(&key).unwrap();

        match *data {
            StorageValue::Set(ref mut d) => {
                let result = d.insert(member);
                Ok(result)
            },
            _ => Err(StorageError::InternalError)
        }
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
        let mut storage = Storage::new();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
    }

    #[test]
    fn test_get_with_no_data() {
        let mut storage = Storage::new();

        assert_eq!(storage.get("test".to_string()).is_err(), true);
    }

    #[test]
    fn test_set_overwrite_key() {
        let mut storage = Storage::new();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
        assert_eq!(storage.set("test".to_string(), "test2".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test2".to_string());
        assert_eq!(storage.count_keys(), 1);
    }

    #[test]
    fn test_delete_key() {
        let mut storage = Storage::new();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).unwrap().to_string(), "test".to_string());
        assert_eq!(storage.del("test".to_string()).unwrap(), true);
        assert_eq!(storage.get("test".to_string()).is_err(), true);
        assert_eq!(storage.count_keys(), 0);
    }

    #[test]
    fn test_count_keys() {
        let mut storage = Storage::new();

        assert_eq!(storage.set("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.set("test2".to_string(), "test2".to_string()).unwrap(), true);

        assert_eq!(storage.count_keys(), 2);
    }

    #[test]
    fn test_sadd() {
        let mut storage = Storage::new();

        assert_eq!(storage.sadd("test".to_string(), "test".to_string()).unwrap(), true);
    }

    #[test]
    fn test_sismember() {
        let mut storage = Storage::new();

        assert_eq!(storage.sadd("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.sismember("test".to_string(), "test".to_string()).unwrap(), true);
    }

    #[test]
    fn test_smembers() {
        let mut storage = Storage::new();

        assert_eq!(storage.sadd("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.sadd("test".to_string(), "test2".to_string()).unwrap(), true);

        let set = storage.smembers("test".to_string()).unwrap();

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_srem() {
        let mut storage = Storage::new();

        assert_eq!(storage.sadd("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.smembers("test".to_string()).unwrap().len(), 1);

        assert_eq!(storage.srem("test".to_string(), "test".to_string()).unwrap(), true);
        assert_eq!(storage.smembers("test".to_string()).unwrap().len(), 0);
    }
}