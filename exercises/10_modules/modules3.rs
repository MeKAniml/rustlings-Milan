// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// TODO: Complete this use statement
/// so as you can use 'use' for call a module from anywhere and rust library, we use std::time to call SystemTime and UNIX_EPOCH from the std::time module
/// to explain the syntax, we use 'use' to call the module, and then we use '::' to call the module from the std library, and then from time module we '::' with {} to call different things inside of this module
use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
