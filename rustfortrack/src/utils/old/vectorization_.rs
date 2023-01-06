use crate::utils::write_matrix_file;
use image;
use imageproc;
use opencv::{
    prelude::*,
    imgcodecs,
    imgproc,
    core,
};
use geo::{Point,LineString, Polygon};


pub fn vectorize(clusters : Vec<Vec<f32>>, clust_threshold: Vec<f32>, data_x_dim : i32, data_y_dim : i32) {

    // Get size of clust_threshold vector
    let clust_threshold_size = clust_threshold.len();
    // Cast clust_threshold_size to i32
    let clust_threshold_size = clust_threshold_size as i32;

    // Loop through clust_threshold vector
    for i in 0..clust_threshold_size {
        // Call generate_matrix function to create empty matrix
        let mut matrix = generate_matrix(data_x_dim, data_y_dim);
        // Fill matrix with clusters
        matrix = fill_matrix(matrix, clusters.clone(), &i);
        // Normalize matrix values between 0 and 255
        matrix = normalize_matrix(matrix);
        // Convert matrix to image
        let img = image::ImageBuffer::from_fn(data_x_dim as u32, data_y_dim as u32, |x, y| {
            let pixel = matrix[x as usize][y as usize];
            image::Luma([pixel as u8])
        });

        // Find contours in the image and store them in a variable
        let mut contours = opencv::types::VectorOfVectorOfPoint::new();

        // Convert img to use in find_contours function with dimensions data_x_dim and data_y_dim
        let img = core::Mat::from_slice(&img).unwrap();
        let img = img.reshape(0, data_x_dim).unwrap();
        let img = img.reshape(0, data_y_dim).unwrap();
        
        // Use funciont find_contours to find contours in the image
        imgproc::find_contours(&img, &mut contours, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, core::Point::new(0, 0)).unwrap();

        for i in 0..contours.len() {
            let contour = contours.get(i).unwrap();
            // Create a vector to store the contour points as vec! of tuples
            let mut contour_vector : Vec<(i32, i32)> = Vec::new();

            for j in 0..contour.len() {
                let point = contour.get(j).unwrap();
                let x = point.x;
                let y = point.y;
                contour_vector.push((x, y));
            }
            
            // Convert the vector of points into an iterator of Point objects.
            let point_iter = contour_vector.into_iter().map(|(x, y)| Point::new(x, y));
            // Create a LineString from the iterator of points.
            let linestring = LineString::from_iter(point_iter);
            // Create a Polygon from the LineString.
            let polygon = Polygon::new(linestring, vec![]);
            // Print the Polygon.
            println!("{:?}", polygon);


        }

        // Exit program
        // std::process::exit(0);

        // // Save image
        // img.save(format!("matrix_{}.png", i)).unwrap();

        // Call write_matrix_file function to write matrix to csv file
        // write_matrix_file::write_matrix(matrix, &i.to_string());

    }
}



// Create a function to generate empty matrix
pub fn generate_matrix(data_x_dim : i32, data_y_dim : i32) -> Vec<Vec<i32>> {
    // Create a matrix with x rows and y columns
    let mut matrix : Vec<Vec<i32>> = Vec::new();
    for j in 0..data_x_dim {
        let mut row : Vec<i32> = Vec::new();
        for k in 0..data_y_dim {
            row.push(0);
        }
        matrix.push(row);
    }
    matrix
}


fn fill_matrix(empty_matrix : Vec<Vec<i32>>, clusters : Vec<Vec<f32>>, threshold: &i32)  -> Vec<Vec<i32>> {
    // Create a matrix with x rows and y columns
    let mut filled_matrix : Vec<Vec<i32>> = empty_matrix;

    // Loop through clusters vector
    for i in 0..clusters.len() {
        // Check if cluster is in threshold
        if clusters[i][3] == *threshold as f32 {
            // Get x and y values
            let x = clusters[i][0] as i32;
            let y = clusters[i][1] as i32;
            // Get cluster number
            let cluster = clusters[i][4] as i32;
            // Set value in matrix
            filled_matrix[x as usize][y as usize] = cluster;
        }
    }
    filled_matrix
}

fn normalize_matrix(matrix : Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Get max value in matrix
    let max = matrix.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    // Get min value in matrix
    let min = matrix.iter().map(|row| row.iter().min().unwrap()).min().unwrap();
    // Get range of values
    let range = max - min;
    // Create normalized matrix
    let mut normalized_matrix : Vec<Vec<i32>> = Vec::new();
    // Loop through matrix
    for i in 0..matrix.len() {
        let mut row : Vec<i32> = Vec::new();
        for j in 0..matrix[i].len() {
            // Normalize value
            let normalized_value = (matrix[i][j] - min) * 255 / range;
            // Push normalized value to row
            row.push(normalized_value);
        }
        // Push row to normalized matrix
        normalized_matrix.push(row);
    }
    normalized_matrix
}


extern crate scoped_threadpool;
use std::sync::Arc;
use scoped_threadpool::Pool;

fn main() {
    let mut pool = Pool::new(4);
    let mut results = Vec::new();

    let results = Arc::new(results);

    pool.scoped(|scope| {
        // This closure will be executed in parallel by 4 worker threads.
        for i in 0..8 {
            let results = results.clone();

            scope.execute(move || {
                // This is the closure that will be executed in parallel.
                // Call the function here and store the result in the results vector.
                let result = my_function(i);
                results.lock().unwrap().push(result);
            });
        }
    });

    // The results vector now contains the results of the function calls.
    println!("Results: {:?}", *results);
}

fn my_function(i: i32) -> i32 {
    // This is a dummy function that just returns the input value.
    i
}
