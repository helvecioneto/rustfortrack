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
use geo::{Point,LineString, Polygon};

pub fn cluster(_data : Vec<Vec<f32>>, data_x_dim : i32, data_y_dim : i32, _clust_threshold: Vec<f32>, _clust_minsize : Vec<i32>, _operator : String) {

    //////////////// Multithreaded version ////////////////
    // Create a pool of threads
    let mut pool = Pool::new((_clust_threshold.len() as usize).try_into().unwrap());
    // Create a vector to store the results
    let mut results = Vec::<Vec<Vec<i32>>>::new();
    let results = Arc::new(Mutex::new(results));
    // Pool scope
    pool.scoped(|scope| {
        // loop through clust_threshold vector
        for threshold_vector in 0.._clust_threshold.len() {
            // Make usable variables
            let current_threshold = _clust_threshold[threshold_vector] as f32;
            let data = _data.clone();
            let _operator = _operator.clone();
            let _clust_minsize = _clust_minsize.clone();
            let data_x_dim = data_x_dim;
            let data_y_dim = data_y_dim;
                        
            // Clone the results vector to store the results
            let results = results.clone();
            // Execute the function in a thread
            scope.execute(move || {
                // Call the function here and store the result in the results vector.
                let result = process_threshold(data, current_threshold, _operator, data_x_dim, data_y_dim);
                let mut results = results.lock().unwrap();
                results.push(result);
            });
        }
    });
    
    // println!("Results: {:?}", *results.lock().unwrap());

    // //////////////// Single threaded version ////////////////
    // // loop through clust_threshold vector
    // for threshold_vector in 0.._clust_threshold_len {
    //     // Cast _clust_threshold[threshold] to f32
    //     let current_threshold = _clust_threshold[threshold_vector] as f32;
    //     // Get xy of thresholded data
    //     let _xy_data = thresholding(_data.clone(), current_threshold, _operator.clone());
    //     // Create a local empty matrix
    //     let mut _local_matrix = generate_matrix(data_x_dim, data_y_dim);
    //     // Binary matrix
    //     _local_matrix = fill_matrix(_local_matrix, _xy_data, 1);
    //     // matrix_to_image
    //     let _local_matrix = matrix_to_image(_local_matrix, data_x_dim, data_y_dim);
    //     // find contours using function get_contours
    //     let _contours = get_contours(_local_matrix, data_x_dim, data_y_dim);
    //     // print threshold and countour length
    //     println!("Threshold: {}, Contour Length: {}", current_threshold, _contours.len());
    // }

}


fn process_threshold(data : Vec<Vec<f32>>,
                     clust_threshold : f32,
                     operator : String,
                     data_x_dim : i32,
                     data_y_dim : i32) -> Vec<Vec<i32>> {

    // Call threshold function
    let xy_data = thresholding(data, clust_threshold, operator);
    // Call create a local empty matrix
    let mut local_matrix = generate_matrix(data_x_dim, data_y_dim);
    // Call binary matrix
    local_matrix = fill_matrix(local_matrix, xy_data.clone(), 1);
    // Call matrix_to_image
    let local_matrix = matrix_to_image(local_matrix, data_x_dim, data_y_dim);
    // Call find contours using function get_contours
    let contours = get_contours(local_matrix, data_x_dim, data_y_dim);
    // Call coordinates to polygons
    let polygons = coords_to_polygon(contours);
    println!("Polygons: {:?}", polygons);

    return xy_data;
}


// Create function to threshold data
fn thresholding(data : Vec<Vec<f32>>, clust_threshold : f32, operator : String) -> Vec<Vec<i32>>{

    // empty vectors to store x and y coordinates
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

// Create a function to transform in Polygon
fn coords_to_polygon(contours : opencv::types::VectorOfVectorOfPoint) -> Vec<Polygon<i32>>{

    // Create a vector to store the polygons type is Polygon
    let mut polygons : Vec<Polygon<i32>> = Vec::new();

    // Loop through contours
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
        // Push the polygon into the vector
        polygons.push(polygon);
    }    
    return polygons;
}
