use std::collections::HashMap; // HashMap
use crate::utils::files_list::files_list; // Import mod files_list from utils
use crate::utils::files_timestamp::files_stamp; // Import mod files_timestamp from utils

pub fn track_mode(name_list_store: &HashMap<String, String>) {
    // Print track mode
    println!("\n\nTrack mode has been started");

    // Get DATA_INPUT path and store it into a variable
    let data_input = name_list_store.get("DATA_INPUT").unwrap();
    // Get DATA_EXT and store it into a variable
    let data_ext = name_list_store.get("DATA_EXT").unwrap();
    // Get TRACK_START and store it into a variable
    let track_start = name_list_store.get("TRACK_START").unwrap();
    // Get TRACK_END and store it into a variable
    let track_end = name_list_store.get("TRACK_END").unwrap();

    // Call files_list function
    let files = files_list(data_input.to_string(), data_ext.to_string());

    // Call timestamp function
    let files_stamp = files_stamp(files);

}