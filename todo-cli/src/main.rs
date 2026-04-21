fn main() {
    println!("Hello, world!");
}
// This is the main src file main code will be present here


// Strict memory management and error handling will be implemented in this file. The main function will serve as the entry point for the application, and additional modules and functions will be defined to handle various functionalities of the todo CLI application.

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
}