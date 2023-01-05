use std::collections::HashMap; // HashMap
use chrono::NaiveDateTime;
use crate::utils::files_list::files_list; // Import mod files_list from utils
use crate::utils::files_timestamp::files_stamp; // Import mod files_timestamp from utils
use crate::utils::read_bin::read_binary; // Import mod read_bin from utils
use crate::utils::thresholding::threshold; // Import mod threshold from utils
use crate::utils::clustering::cluster; // Import mod clustering from utils
use crate::utils::vectorization::vectorize; // Import mod vectorization from utils


pub fn track_mode(name_list_store: &HashMap<String, String>) {
    // Print track mode
    println!("\n\nTrack mode has been started");

    // Get DATA_INPUT path and store it into a variable
    let data_input = name_list_store.get("DATA_INPUT").unwrap();
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
    // Get CLUST_THRESHOLD and store it into a variable
    let clust_threshold = name_list_store.get("CLUST_THRESHOLD").unwrap();
    // Get CLUST_MINSIZE and store it into a variable
    let clust_minsize = name_list_store.get("CLUST_MINSIZE").unwrap();
    // Get OPERATOR and store it into a variable
    let operator = name_list_store.get("OPERATOR").unwrap();

    // Format variables
    //Split clust_threshold into a float vector
    let clust_threshold: Vec<f32> = clust_threshold.split(",").map(|s| s.parse().unwrap()).collect();
    // Split clust_minsize into a int vector
    let clust_minsize: Vec<i32> = clust_minsize.split(",").map(|s| s.parse().unwrap()).collect();

    // Call files_list function
    let files = files_list(data_input.to_string());

    // Call timestamp function to filter files
    let filtered_files = files_stamp(files, track_start.to_string(), 
                                  track_end.to_string(),
                                  track_interval.to_string().parse::<i32>().unwrap());

        
    // Begin track process
    for i in 0..filtered_files.len() {
        // Get timestamp
        let timestamp = filtered_files[i].split("/").last().unwrap().split(".").next().unwrap();
        let timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y%m%d_%H%M%S").unwrap();

        // Print timestamp
        println!("Processing: {} ", timestamp);

        // Call read_bin function
        let data = read_binary(filtered_files[i].to_string(),
                               data_x_dim.to_string(),
                               data_y_dim.to_string());

        // Call threshold function
        let threshold_data = threshold(data, clust_threshold.clone(),operator.to_string());

        // Call cluster function
        let cluster_data = cluster(threshold_data, clust_threshold.clone(), clust_minsize.clone());

        // Call vectorize function
        vectorize(cluster_data, clust_threshold.clone(), data_x_dim.to_string().parse::<i32>().unwrap(), data_y_dim.to_string().parse::<i32>().unwrap());

    }
}

