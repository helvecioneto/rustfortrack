extern crate scoped_threadpool;
use std::sync::{Arc, Mutex};
use scoped_threadpool::Pool;
use image;
// use imageproc;
use opencv::{
    prelude::*,
    // imgcodecs,
    imgproc,
    core,
};

pub fn cluster(_data : Vec<Vec<f32>>, data_x_dim : i32, data_y_dim : i32, _clust_threshold: Vec<f32>, _clust_minsize : Vec<i32>, _operator : String) {

    // Get _clust_threshold.len()
    let _clust_threshold_len = _clust_threshold.len();
    // Create a pool of threads
    let mut pool = Pool::new((_clust_threshold_len as usize).try_into().unwrap());
    // Create a vector to store the results
    let mut results = Vec::<i32>::new();
    let results = Arc::new(Mutex::new(results));

    // Pool scope
    pool.scoped(|scope| {
        // loop through clust_threshold vector
        for threshold_vector in 0.._clust_threshold_len {
            // Cast _clust_threshold[threshold] to f32
            let current_threshold = _clust_threshold[threshold_vector] as i32;
            // Clone the results vector
            let results = results.clone();
            // Execute the function in a thread
            scope.execute(move || {
                // Call the function here and store the result in the results vector.
                let result = process_threshold(current_threshold);
                let mut results = results.lock().unwrap();
                results.push(result);

            });
        }
    });
    println!("Results: {:?}", *results.lock().unwrap());

    // // loop through clust_threshold vector
    // for threshold_vector in 0.._clust_threshold_len {
    //     // Cast _clust_threshold[threshold] to f32
    //     let current_threshold = _clust_threshold[threshold_vector] as f32;
        // // Get xy of thresholded data
        // let _xy_data = thresholding(_data.clone(), current_threshold, _operator.clone());
        // // Create a local empty matrix
        // let mut _local_matrix = generate_matrix(data_x_dim, data_y_dim);
        // // Binary matrix
        // _local_matrix = fill_matrix(_local_matrix, _xy_data, 1);
        // // matrix_to_image
        // let _local_matrix = matrix_to_image(_local_matrix, data_x_dim, data_y_dim);
        // // find contours using function get_contours
        // let _contours = get_contours(_local_matrix, data_x_dim, data_y_dim);
        // // print threshold and countour length
        // println!("Threshold: {}, Contour Length: {}", current_threshold, _contours.len());

    // }
}


fn process_threshold(i: i32) -> i32 {
    // Create a vector to store the results
    let mut results = Vec::<i32>::new();
    // Push the result into the vector
    results.push(i);
    // Return the result
    return i;
}


// Create function to threshold data
fn thresholding(data : Vec<Vec<f32>>, clust_threshold : f32, operator : String) -> Vec<Vec<i32>>{

    let mut x : Vec<i32> = Vec::new();
    let mut y : Vec<i32> = Vec::new();

    // loop through data vector
    for j in 0..data.len() {
        for k in 0..data[j].len() {
            // Check operator equal to or greater than
            if operator == ">=" {
                if data[j][k] >= clust_threshold {
                    // push x, y, threshold, and value into vector
                    x.push(k as i32);
                    y.push(j as i32);
                }
            // Check operator equal to or less than
            } else if operator == "<=" {
                if data[j][k] <= clust_threshold {
                    // push x, y, threshold, and value into vector
                    x.push(k as i32);
                    y.push(j as i32);
                }
            // Check operator equal to greater
            } else if operator == ">" {
                if data[j][k] > clust_threshold {
                    // push x, y, threshold, and value into vector
                    x.push(k as i32);
                    y.push(j as i32);
                }
            // Check operator equal to less
            } else if operator == "<" {
                if data[j][k] < clust_threshold {
                    // push x, y, threshold, and value into vector
                    x.push(k as i32);
                    y.push(j as i32);
                }
            // Check operator equal to equal
            } else if operator == "==" {
                if data[j][k] == clust_threshold {
                    // push x, y, threshold, and value into vector
                    x.push(k as i32);
                    y.push(j as i32);
                }
            }
        }
    }

    // Concatenate into a vertical vector of i32
    let thresholded_points = vec![x, y];

    return thresholded_points;
}


// Create a function to generate empty matrix
fn generate_matrix(data_x_dim : i32, data_y_dim : i32) -> Vec<Vec<i32>> {
    // Create a matrix with x rows and y columns
    let mut matrix : Vec<Vec<i32>> = Vec::new();
    for _x in 0..data_x_dim {
        let mut row : Vec<i32> = Vec::new();
        for _y in 0..data_y_dim {
            row.push(0);
        }
        matrix.push(row);
    }
    return matrix;
}

// Create a function to fill matrix
fn fill_matrix(mut matrix : Vec<Vec<i32>>, xy_data : Vec<Vec<i32>>, value : i32) -> Vec<Vec<i32>> {
    // loop through xy_data vector
    for j in 0..xy_data[0].len() {
        // Fill matrix with value
        matrix[xy_data[0][j] as usize][xy_data[1][j] as usize] = value;
    }
    return matrix;
}

// Create a function to convert matrix to image
fn matrix_to_image(_local_matrix : Vec<Vec<i32>>, data_x_dim : i32, data_y_dim : i32) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    
    // Convert binary matrix to image type
    let _local_matrix = image::ImageBuffer::from_fn(data_x_dim as u32, data_y_dim as u32, |x, y| {
        let pixel = _local_matrix[x as usize][y as usize];
        image::Luma([pixel as u8])
    });

    return _local_matrix;
}

// Create function to find countours
fn get_contours(_local_matrix : image::ImageBuffer<image::Luma<u8>, Vec<u8>>, data_x_dim : i32, data_y_dim : i32) -> opencv::types::VectorOfVectorOfPoint {
    // Reshape matrix to 2D vector
    let _local_matrix = core::Mat::from_slice(&_local_matrix).unwrap();
    let _local_matrix = _local_matrix.reshape(0, data_x_dim).unwrap();
    let _local_matrix = _local_matrix.reshape(0, data_y_dim).unwrap();

    // Countours variable to store Point vector
    let mut _contours = opencv::types::VectorOfVectorOfPoint::new();

    // Find contours of binary matrix
    imgproc::find_contours(&_local_matrix, &mut _contours, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, core::Point::new(0, 0)).unwrap();

    return _contours;
}