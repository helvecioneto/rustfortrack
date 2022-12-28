// import chrono
use chrono::NaiveDateTime;

pub fn files_stamp(files_list: Vec<String>,track_start: String, track_end: String, track_int: i32) -> Vec<String> {

    // Convert track_start and track_end to datetime
    let track_start = NaiveDateTime::parse_from_str(&track_start, "%Y-%m-%d %H:%M:%S").unwrap();
    let track_end = NaiveDateTime::parse_from_str(&track_end, "%Y-%m-%d %H:%M:%S").unwrap();

    // Create a vector starting from track_start to track_end with track_int interval
    let mut track_time: Vec<NaiveDateTime> = Vec::new();
    let mut track_time_temp = track_start;
    while track_time_temp <= track_end {
        track_time.push(track_time_temp);
        track_time_temp = track_time_temp + chrono::Duration::seconds(track_int as i64);
    }

    // Create a vector with files timestamp
    let mut files_filtered: Vec<String> = Vec::new();

    // Loop over files_list
    for file in files_list.iter() {
        let file_name = file.split("/").last().unwrap();
        let timestmp = NaiveDateTime::parse_from_str(&file_name[0..19], "%Y%m%d_%H%M%S.bin").unwrap();
        if track_time.contains(&timestmp) {
            // Store file in files_stamp
            files_filtered.push(file.to_string());
        }
    }

    return files_filtered;
}