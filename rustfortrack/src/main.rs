// Author: Helvecio Neto
// Date: 2022-12-27
// Description: Rust for Track - Main program

// Import libraries
use std::env; // Get command line arguments
use std::fs::File; // File I/O
use ini::Ini;
mod utils; // Import utils module

#[allow(unused_variables)] // Allow unused variables

fn main() {
    // Get imput parameters from command line
    let args: Vec<String> = env::args().collect();
    // Check if args is empty
    if args.len() == 1 {
        // Print --help
        println!("Usage: rustfortrack <name_list_path> <track/forecast>");
        return;
    }

    let name_list = &args[1]; // Get name_list path
    let mode = &args[2]; // Get mode (track/forecast)

    // Check if name_list is a file and do not exists
    let file = File::open(name_list);
    match file {
        // If file exists do nothing
        Ok(_) => (),
        // If file does not exists print error and exit
        Err(_) => {
            println!("Error: File {} does not exists", name_list);
            return;
        }
    }

    // Init program
    utils::main_bar::main_print(); // Print main bar

    // Read name_list file
    let conf = Ini::load_from_file(name_list).unwrap();

    // iterating
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        }
    }


}
