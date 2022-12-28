extern crate glob;

pub fn files_list(path: String, extension: String) -> Vec<String> {

    // mount pattern
    let pattern = format!("{}**/*{}", path, extension);
    // get files and add to vector
    let mut files: Vec<String> = Vec::new();
    for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }

    // Check if files is empty
    if files.is_empty() {
        // Print error and exit
        println!("Error: No files found into {} directory", path);
        // Call exit function
        std::process::exit(0);
    }
    
    // Sort files by name
    files.sort();

    // Return files
    return files;
}