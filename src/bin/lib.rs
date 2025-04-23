// Automated Testing Library for Rust Applications

// Developer: Kyle Brady
// Project Name: 'WackyATL'
// Description: Testing and Benchmarking library for Rust Tests and Builds

use ServerMap;

#[cfg(test)]
pub mod Wacky {

    // For running tests against the modules
    // used to build this HTTP Server
    pub struct MapTester;
    pub struct StorageTester;
    pub struct ResponseTester;
    pub struct RequestTester;

    impl MapTester {
        // Test to see if we can get an item
        // via a literal key in the ServerMap

        #[test]
        fn t_get_success() {
            // For Str/Int KV Pairs
            let key: String = "testkey";
            let val: i32 = 1;

            // Adding our KVPair values to the newly generated map
            let mut serv_map: ServerMap<String, i32> = ServerMap::generate();
            serv_map.push(key.clone(), value);

            let result = serv_map.request(key).unwrap();
            assert_eq!(result, value);
        }
    }
}
