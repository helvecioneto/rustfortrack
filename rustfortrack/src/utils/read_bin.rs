use std::fs::File; // File
use std::io::Read; // Read

pub fn read_binary(path: String, data_x_dim: String, data_y_dim: String) -> Vec<Vec<f32>> {
    // Open file
    let mut file = File::open(path).expect("Unable to open file");
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
    return matrix;
}