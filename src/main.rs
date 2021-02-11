extern crate ctloader;
use std::process;

fn main() {
    let config = match ctloader::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = ctloader::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
