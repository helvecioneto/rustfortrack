use std::collections::HashMap; // HashMap
use std::fs::File; // File
use std::io::Read; // Read
use crate::utils::files_list::files_list; // Import mod files_list from utils
use crate::utils::files_timestamp::files_stamp; // Import mod files_timestamp from utils

pub fn track_mode(name_list_store: &HashMap<String, String>) {
    // Print track mode
    println!("\n\nTrack mode has been started");

    // Get DATA_INPUT path and store it into a variable
    let data_input = name_list_store.get("DATA_INPUT").unwrap();
    // Get DATA_EXT and store it into a variable
    let data_ext = name_list_store.get("DATA_EXT").unwrap();
    // Get DATA_X_DIM and store it into a variable
    let data_x_dim = name_list_store.get("DATA_X_DIM").unwrap();
    // Get DATA_Y_DIM and store it into a variable
    let data_y_dim = name_list_store.get("DATA_Y_DIM").unwrap();
    // Get TRACK_START and store it into a variable
    let track_start = name_list_store.get("TRACK_START").unwrap();
    // Get TRACK_END and store it into a variable
    let track_end = name_list_store.get("TRACK_END").unwrap();
    // Get TRACK_INTERVAL and store it into a variable
    let track_interval = name_list_store.get("TRACK_INTERVAL").unwrap();

    // Call files_list function
    let files = files_list(data_input.to_string(), data_ext.to_string());

    // Call timestamp function to filter files
    let filtered_files = files_stamp(files, track_start.to_string(), 
                                  track_end.to_string(),
                                  track_interval.to_string().parse::<i32>().unwrap());


    let first_file = filtered_files[0].clone();
    
    // Open file
    let mut file = File::open(first_file).expect("Unable to open file");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Unable to read data");

    // Create a main vector to store data
    let mut main_vector = Vec::new();

    for i in (0..buffer.len()).step_by(4) {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&buffer[i..i+4]);
        let f = f32::from_le_bytes(bytes);
        main_vector.push(f);
    }

    // Create a matrix of f32 with DATA_X_DIM and DATA_Y_DIM
    let mut matrix = vec![vec![0.0; data_x_dim.to_string().parse::<usize>().unwrap()]; 
                                    data_y_dim.to_string().parse::<usize>().unwrap()];
    
    // Store data into matrix
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            matrix[i][j] = main_vector[i * matrix[i].len() + j];
        }
    }

    println!(" {} ", matrix[201][120]);

}

