use std::{collections::HashMap, any::Any};



fn main() {
    let mut map = HashMap::new(); //  creates a new empty HashMap.

    map.insert("anas".to_string() , "0"); 


    
    map.insert("anas".to_string() , "1100"); // inserts a key-value pair into the HashMap. If the key already exists, the value is updated.
    
    // let anas = map.get("anas").unwrap();
    
    map.entry("jaidi".to_string()).or_insert("anas");
    
    let hello = "anas jaidi anas jaidi anas anas";
    let mut words = HashMap::new();
    for w in hello.split_whitespace() {
      let c = words.entry(w).or_insert(0);
      *c += 1;
    }

    println!("{:?}", words);

  }