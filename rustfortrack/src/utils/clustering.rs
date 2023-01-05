use ndarray::{Array2};
use petal_neighbors::distance::Euclidean;
use petal_clustering::{Dbscan, Fit};

pub fn cluster(points : Vec<Vec<f32>>, clust_threshold: Vec<f32>, clust_minsize : Vec<i32>) -> Vec<Vec<f32>>{

    // Get Threshold Level Vector
    let thld_vec = points[0].clone();
    // Get X Vector
    let x_vec = points[1].clone();
    // Get Y Vector
    let y_vec = points[2].clone();
    // Get Value Vector
    let val_vec = points[3].clone();

    // Set epsilon value for clustering
    let eps = 1.5;
    // Set min points for clustering
    let min_pts = 3;

    // Create a object for Dbscan
    let mut clustering = Dbscan::new(eps,min_pts, Euclidean::default());

    // Create new array with the difference of rows
    let mut output_matrix : Vec<Vec<f32>> = Vec::new();

    // Loop through clust_threshold vector
    for i in 0..clust_threshold.len() {
        // Get position of threshold level where equal to threshold level
        let mut thld_pos : Vec<usize> = Vec::new();
        for j in 0..thld_vec.len() {
            if thld_vec[j] == clust_threshold[i] {
                thld_pos.push(j);
            }
        }

        // Create array with x and y values
        let mut arr: Array2<f32> = Array2::zeros((thld_pos.len(), 2));
        for j in 0..thld_pos.len() {
            arr[[j, 0]] = x_vec[thld_pos[j]] as f32;
            arr[[j, 1]] = y_vec[thld_pos[j]] as f32;
        }
        
        // Fit clustering
        let clusters = clustering.fit(&arr).0;

        // Loop through clusters Hash and print
        for (key, value) in clusters.iter() {
            for j in 0..value.len() {
                // Check if all values are different from 0
                if x_vec[thld_pos[value[j]]] != 0.0 && y_vec[thld_pos[value[j]]] != 0.0 && val_vec[thld_pos[value[j]]] != 0.0 {
                    // Increment 1 to key
                    let cluster_id = key + 1;
                    // Create new row
                    let mut row : Vec<f32> = Vec::new();
                    row.push(x_vec[thld_pos[value[j]]] as f32);
                    row.push(y_vec[thld_pos[value[j]]] as f32);
                    row.push(val_vec[thld_pos[value[j]]] as f32);
                    row.push(i as f32);
                    row.push(cluster_id as f32);
                    output_matrix.push(row);
                }
            }
        }
    }

    // Sort output_matrix array by last column
    output_matrix.sort_by(|a, b| a[4].partial_cmp(&b[4]).unwrap());

    return output_matrix;
}
