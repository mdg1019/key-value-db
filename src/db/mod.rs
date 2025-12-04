use std::any::Any;
use std::collections::HashMap;

pub trait Database {
    fn insert(&mut self, key: impl Into<String>, value: Box<dyn Any>);
    fn get_as<T: 'static>(&self, key: &str) -> Option<&T>;
    fn remove(&mut self, key: &str) -> Option<Box<dyn Any>>;
}

pub struct InMemoryDb {
    data: HashMap<String, Box<dyn Any>>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        InMemoryDb {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&Box<dyn Any>> {
        self.data.get(key)
    }
}

impl Database for InMemoryDb {
    fn insert(&mut self, key: impl Into<String>, value: Box<dyn Any>) {
        self.data.insert(key.into(), value);
    }

    fn get_as<T: 'static>(&self, key: &str) -> Option<&T> {
        self.get(key)?.downcast_ref::<T>()
    }  

    fn remove(&mut self, key: &str) -> Option<Box<dyn Any>> {
        self.data.remove(key)
    }
}

impl InMemoryDb {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_db_should_be_empty_when_created() {   
        let db = InMemoryDb::new();
        assert!(db.data.is_empty());
    }

    #[test]
    fn insert_a_value_should_store_it_in_db() {
        let mut db = InMemoryDb::new();
        assert!(db.data.is_empty());
        db.insert("key1", Box::new(123i32));
        let value = db.data.get("key1");
        assert_eq!(value.unwrap().downcast_ref::<i32>(), Some(&123i32));
    }

    #[test]
    fn overwrite_existing_value_should_update_it() {
        let mut db = InMemoryDb::new();
        db.insert("key1", Box::new(123i32));
        let value = db.data.get("key1");
        assert_eq!(value.unwrap().downcast_ref::<i32>(), Some(&123i32));
        db.insert("key1", Box::new(456i32));
        let value = db.data.get("key1");
        assert_eq!(value.unwrap().downcast_ref::<i32>(), Some(&456i32));
    }

    #[test]
    fn get_existing_value_should_return_it() {
        let mut db = InMemoryDb::new();
        db.insert("key1", Box::new(123i32));
        let value = db.get_as::<i32>("key1").unwrap();
        assert_eq!(value, &123i32);
    }

    #[test]
    fn get_non_existing_value_should_return_none() {
        let db = InMemoryDb::new();
        let value = db.get_as::<i32>("nonexistent");
        assert!(value.is_none());
    }

    #[test]
    fn remove_existing_value_should_delete_it() {
        let mut db = InMemoryDb::new();
        db.insert("key1", Box::new(123i32));
        let removed = db.remove("key1");
        assert_eq!(removed.unwrap().downcast_ref::<i32>(), Some(&123i32));
        assert!(db.get_as::<i32>("key1").is_none());
    }

    #[test]
    fn remove_non_existing_value_should_return_none() {
        let mut db = InMemoryDb::new();
        let removed = db.remove("nonexistent");
        assert!(removed.is_none());
    }
}