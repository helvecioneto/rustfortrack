use std::fs::File;
use std::io::Write;

pub fn write_matrix(matrix: Vec<Vec<i32>>, name: &str) {
    // Write 241x241 matrix to csv file
    let mut file = File::create(format!("matrix_{}.csv", name)).unwrap();
    for j in 0..matrix.len() {
        for k in 0..matrix[j].len() {
            let _ = file.write_all(format!("{},", matrix[j][k]).as_bytes());
        }
        let _ = file.write_all(format!("\n").as_bytes());
    }
}