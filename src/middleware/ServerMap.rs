// Hash Map Implementation for handling routing and HTTP Headers

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Overall Server crate
use mod Server;

// Testing Lib
use Wacky::MapTester;

// Maximum storage size
const MAP_MAX_SIZE: u64 = 256;

// Server map that tracks the size and key-elements
pub struct ServerMap<Key, Val> {
    map_size: usize,
    map_arr: [Option<KeyVal<Key, Val>>; MAP_MAX_SIZE as usize],
    identifier: u16,
}

// Processing for the current K/V Pair and the next available
// option that is presented in the hashmap
#[derive(Clone)]
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

impl <Key: Clone + Hash + PartialEq, Val> ServerMap<Key, Val> {

    const server_conf: Option<KVPairs<Key, Val> = None;
    pub fn generate() -> ServerMap<Key, Val> {
        ServerMap { 
            map_size: 0, 
            map_arr: [Self::server_conf; MAP_MAX_SIZE as usize],
        }
    }
    
    // --> CRUD Operations <--

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
            Some(_) => self.check_key(key, position as usize);
            None => None,
        }\
    }

    // Remove a value and handle the empty container
    pub fn remove(&self, key: Key) -> Option<Val> {
        let hash_handle: u64 = hash_handler(key.clone());
        let position: u64 = hash_handle % MAP_MAX_SIZE;

        match &self.map_arr[position as usize] {
            Some(_) => self.rm_check_key(key, position as usize),
            None => None,
        }
    }

    // Loops through and clears all data - /handling memory
    priv fn clear(&mut self) {
        todo!()
    }

    // Check if key exists in the map already
    // if it exists it will return the corresponding val
    // --> Explicitly used for checking when adding an item
    // --> This will be refactored into a more robust function
    fn check_key(&self, key: Key, position: usize) -> Option<Val> {
        let mut current_pos = self.map_arr[position].as_ref().unwrap();

        // If found - return value
        if current_pos == key {
            return Some(current_pos.value);
        }

        // Traversing the LL until we find the key
        while let Some(node) = current_pos.next.as_ref() {
            if node.key == key {
                return Some(node.value);
            }

            current_pos = node; // for each iteration check
        }

        None
    }

    // Check for a keys existence before removal
    // by iteratively performing checks on the map nodes

    // Notice that some functions may appear to have SOME similar behaviour
    // This will all be updated and considered in the refactoring process                                                                                                                                              
    fn rm_check_key(&mut self, key: Key, position: usize) -> Option<Val> {
        let mut current_pos = self.map_arr[position].as_ref().unwrap();

        if current_pos.key == key {
            let pos_val = current_pos.value;

            // Check for next available val in the map
            // and update it to point to
            if let Some(node) = current_pos.next.to_owned() {
                self.map_arr[position] = Some(*node);
            } else {
                self.map_arr[position] = None
            }
            
            // Returns the value held by the node and updates the map size
            self.map_size -= 1;
            return Some(pos_val);
        }

        // Going through the hash table nodes individually
        // until the key specified is found
        while current_pos.next.is_some() {
            let next = current_pos.next.as_mut().unwrap();

            if next.key == key {
                let val = next.value;

                // check if there is a next value i.e != None
                // update the hash table with some arbitray pointer 'ptr'
                if let Some(ptr) = next.next.to_owned() {
                    current_pos.next = Some(Box::new(*ptr));
                } else {
                    current.next = None;
                }

                // return the value to the node at current_pos
                self.map_size -= 1;
                return Some(val);
            }

            // After all checks are complete
            // the current position will move to the next
            // available node that isnt 'None'
            current_pos = current_pos.next.as_mut().unwrap();
        }

        None
    }

    // --> Data Handling functions <--

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

    // --> Resizing and Manipulation <--

    priv fn resize_server_map() {
        // Based on KVPair entries
        // Resizing the hashmap by * 2 when a set boundary
        // is hit. Also taking into account the current
        // stored elements position in the hashmap

        // Since this is a custom impl for
        // the HTTP Server Flags, Routing etc
        // This is not a necessity and will
        // be implemented as a side feature later (unless it is needed)

        todo!()
    }
}