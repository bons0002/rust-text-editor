use app::init;

use std::env;

fn main() {
    // Get the name of the file to be openned from the cli args
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);

    // Run the app
    match init(filename) {
        Ok(_) => (),
        _ => panic!("Unresolved issue"),
    };
}