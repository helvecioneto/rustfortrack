use ndarray::{Array2};
use petal_neighbors::distance::Euclidean;
use petal_clustering::{Dbscan, Fit};

pub fn cluster(points : Vec<Vec<f32>>, clust_threshold: Vec<f32>, clust_minsize : Vec<i32>) -> Array2<f32>{

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

    // Create array to store x,y,threshold, value,cluster as output of function
    let mut clusterized: Array2<f32> = Array2::zeros((val_vec.len(), 5));

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
                clusterized[[value[j], 0]] = x_vec[thld_pos[value[j]]] as f32;
                clusterized[[value[j], 1]] = y_vec[thld_pos[value[j]]] as f32;
                clusterized[[value[j], 2]] = val_vec[thld_pos[value[j]]] as f32;
                clusterized[[value[j], 3]] = i as f32;
                clusterized[[value[j], 4]] = *key as f32;
            }
        }
    }

    // Remove rows of clusterized array that have all values equal to 0
    let mut row_to_remove : Vec<usize> = Vec::new();
    for i in 0..clusterized.nrows() {
        if clusterized.row(i).iter().all(|&x| x == 0.0) {
            row_to_remove.push(i);
        }
    }

    // Get difference between val_vec.len() and row_to_remove.len()
    let diff = val_vec.len() - row_to_remove.len();

    // Create new array with the difference of rows
    let mut clusterized_new: Array2<f32> = Array2::zeros((diff, 5));

    // Add values to new array from clusterized array
    let mut k = 0;
    for i in 0..clusterized.nrows() {
        if !row_to_remove.contains(&i) {
            clusterized_new[[k, 0]] = clusterized[[i, 0]];
            clusterized_new[[k, 1]] = clusterized[[i, 1]];
            clusterized_new[[k, 2]] = clusterized[[i, 2]];
            clusterized_new[[k, 3]] = clusterized[[i, 3]];
            clusterized_new[[k, 4]] = clusterized[[i, 4]];
            k = k + 1;
        }
    }

    return clusterized_new;
}
