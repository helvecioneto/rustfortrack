// Author: Helvecio Neto
// Date: 2022-12-27
// Description: Rust for Track - Main program

// Import libraries
use std::env; // Get command line arguments
use std::fs::File; // File I/O
use ini::Ini; // Read .ini files
use std::collections::HashMap; // HashMap
mod utils; // Import utils module
mod track; // Import track module
mod forecast; // Import forecast module


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
    let config_list = Ini::load_from_file(name_list).unwrap();

    // Create a HashMap to store the name_list
    let mut name_list_store: HashMap<String, String> = HashMap::new();

    // Iterate over the name_list
    for (sec, prop) in &config_list {
        // Check if section is not equals to CONFIG
        if sec.unwrap() != "CONFIG" {
            // print error and exit
            println!("Error: Section CONFIG does not exists into name_list.ini file. Founded name: {}", sec.unwrap());
            return;
        }
        // Iterate over the properties
        for (key, value) in prop.iter() {
            // Insert name in the HashMap
            name_list_store.insert(key.to_string(), value.to_string());
            // Print name
            println!("{}: {}", key, value);
        }
    }

    // Check if mode is track
    if mode == "track" {
        // Call track function
        track::track_mode::track_mode(&name_list_store);
    } else if mode == "forecast" {
        // Call forecast function
        forecast::forecast_mode::forecast_mode(&name_list_store);
    } else {
        // Print error and exit
        println!("Error: Mode {} does not exists", mode);
        return;
    }
}
