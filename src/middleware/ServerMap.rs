// Hash Map Implementation for handling routing and HTTP Headers

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Maximum storage size
const MAP_MAX_SIZE: u64 = 256;

enum FlagType {
    OK = 200,
    GOOD = 300,
    NOT_FOUND = 404,
    FILE_OUTOF_BOUNDS = 507

    // ETC Will implement properly soon
}

// Custom Flag handling for HTTP Req/Res
pub struct Flag {
    flag_type: FlagType,

}

// Server map that tracks the size and key-elements
pub struct ServerMap<Key, Val> {
    map_size: usize,
    map_arr: [Option<KeyVal<Key, Val>>; MAP_MAX_SIZE as usize],
    identifier: u16,
}

// Processing for the current K/V Pair and the next available
// option that is presented in the hashmap
pub struct KeyVal {
    key: Key,
    val: Val,
    next: Option<Box<KeyVal<Key, Val>>>;
}

// Basic container for holding associative pairs
impl <Key, Val> KVPairs<Key, Val> {
    pub fn new(key: Key, val: Val) -> KVPairs<Key, Val> {
        KVPairs { key, val, next: None }
    }
}

// Basic function for hashing KVP keys
fn hash_handler<Key: Hash>(key: Key) -> u64 {
    let mut hash_func = DefaultHasher::new(); // from std
    key.hash(&mut hash_func);
    let return_hash = hash_func.finish();

    return_hash
}

// Linear Probing to stop index clashes
fn probe_map() {
    todo!()
}

impl <Key: Clone + Hash + PartialEq, Val> ServerMap<Key, Val> {

    // Adds an item to the ServerMap via KV Pairs
    pub fn push(&mut self, key: Key, val: Val) -> Option<Val> {
        // The Key is passed through some arbitrary hash function
        // Where it is then handled and operated upon
        // ------------------------------------------
        // The Position of this is determined by the inital
        // size of the ServerMap
        let hash_handle: u64 = hash_handler(key.clone());
        let position = hash_handle % MAP_MAX_SIZE;

        // Linking Val => Key
        match &self.map_arr[position as usize] {
            Some(_) => self.update_new_data(key, val, position as usize),
            None => {
                self.insert_new_data(key, val, position as usize);
                None
            }
        }
    }

    // Returns the requests value from a given key
    // in the ServerMap
    pub fn request(&self, key: Key) -> Option<Val> {
        let hash_handle: u64 = hash_handler(key.clone());
        let position = hash_handle % MAP_MAX_SIZE;

        match &self.map_arr[position as usize] {
            Some(_) => self.
        }
    }

    // Remove a value and handle the empty container
    pub fn remove(&self, key: Key) -> Option<Val> {
        todo!()
    }

    // Loops through and clears all data - /handling memory
    priv fn clear(&mut self) {
        todo!()
    }

    // Data Handling functions 
    fn insert_new_data(&mut self, key: Key, val: Val, position: usize) {
        let data = KVPairs::new(key, val);
        self.map_arr[position] = Some(data);
        self.map_size += 1;
    }

    fn update_new_data(&mut self, key: Key, val: Val, position: usize) -> Option<Val> {
        // By Traversing the Hash map and updating the value
        // based on whether the value is found (exists = update)
        // or by adding a new, unique value to the end of the map

        let mut key_data = self.map_arr[position].unwrap();
        if key_data.key == key {
            // record the previous value of the key passed
            // and replace it with the new value passed
            let prev_val = key_data.value;
            key_data.value = val;

            return Some(prev_val);
        }

        // Needs Refactoring as the same code is used here twice
        let mut current_data = key_data;
        while current_data.next.is_some() {
            let node = current.next.as_mut().unwrap();

            if node.key == key {
                let prev_val = node.value;
                node.value = val;

                return Some(prev_val);
            }

            current_data = node;
        }

        // Defines the new pair we are going to append to the map
        let new_key = KVPairs::new(key, val);

        // Add the box value to th
        current_data.next = Some(Box::new(new_key));
        self.map_size += 1;

        None
    }
}