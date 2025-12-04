mod db;
use key_value_db::db::{Database, InMemoryDb};
use std::collections::HashMap;

fn main() {
    let mut db = InMemoryDb::new();
    println!("Initialized database");

    db.insert("dog", Box::new("annie".to_string()));
    db.insert("age", Box::new(42i32));
    db.insert("height", Box::new(5.9f64));
    db.insert("active", Box::new(true));

    let mut map = HashMap::new();
    map.insert("country".to_string(), "USA".to_string());
    map.insert("city".to_string(), "Seattle".to_string());
    map.insert("language".to_string(), "Rust".to_string());

    db.insert("location_map", Box::new(map));

    if let Some(name) = db.get_as::<String>("dog") {
        println!("Dog name: {}", name);
    }
    
    if let Some(age) = db.get_as::<i32>("age") {
        println!("Age: {}", age);
    }
    
    if let Some(height) = db.get_as::<f64>("height") {
        println!("Height: {}", height);
    }
    
    if let Some(active) = db.get_as::<bool>("active") {
        println!("Active: {}", active);
    }

    if let Some(location_map) = db.get_as::<HashMap<String, String>>("location_map") {
        println!("Location map: {:?}", location_map);
    }
}
