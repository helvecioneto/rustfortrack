use std::fs::File;
use std::io::Write;

// import image
use image;
use imageproc;
// use imageproc::drawing::{draw_contours, draw_polygon};
// use imageproc::polygon::Polygon;


pub fn cluster(points : Vec<Vec<f32>>, clust_threshold: Vec<f32>, clust_minsize : Vec<i32>){

    // Get Threshold Level Vector
    let thld_vec = points[0].clone();
    // Get X Vector
    let x_vec = points[1].clone();
    // Get Y Vector
    let y_vec = points[2].clone();
    // Get Value Vector
    let val_vec = points[3].clone();


    // Loop through clust_threshold vector
    for i in 0..clust_threshold.len() {
        // Get position of threshold level where equal to threshold level
        let mut thld_pos : Vec<usize> = Vec::new();
        for j in 0..thld_vec.len() {
            if thld_vec[j] == clust_threshold[i] {
                thld_pos.push(j);
            }
        }
        // Get x, y, and value vectors where threshold level equal to threshold level
        let mut x : Vec<f32> = Vec::new();
        let mut y : Vec<f32> = Vec::new();
        let mut val : Vec<f32> = Vec::new();
        for j in 0..thld_pos.len() {
            x.push(x_vec[thld_pos[j]]);
            y.push(y_vec[thld_pos[j]]);
            val.push(val_vec[thld_pos[j]]);
        }


        // Set size of matrix 241,241
        let mut matrix : Vec<Vec<i32>> = Vec::new();
        for j in 0..241 {
            let mut row : Vec<i32> = Vec::new();
            for k in 0..241 {
                row.push(0);
            }
            matrix.push(row);
        }

        // Loop through x and y vectors
        for j in 0..x.len() {
            // Set matrix value to 1
            matrix[y[j] as usize][x[j] as usize] = 1;
        }

        // Transform matrix to image
        let mut imgbuf = image::ImageBuffer::new(241, 241);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            if matrix[y as usize][x as usize] == 1 {
                *pixel = image::Luma([255u8]);
            } else {
                *pixel = image::Luma([0u8]);
            }
        }



        // // Write 241x241 matrix to csv file
        // let mut file = File::create("matrix.csv").unwrap();
        // for j in 0..matrix.len() {
        //     for k in 0..matrix[j].len() {
        //         let _ = file.write_all(format!("{},", matrix[j][k]).as_bytes());
        //     }
        //     let _ = file.write_all(format!("\n").as_bytes());
        // }
    }
}
