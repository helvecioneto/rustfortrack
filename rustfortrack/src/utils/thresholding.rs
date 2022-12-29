pub fn threshold(data : Vec<Vec<f32>>, clust_threshold : Vec<f32>, operator : String) -> Vec<Vec<f32>>{
    
    // Create a vector to store x coordinates, y coordinates, threshold, and value
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut threshold = Vec::new();
    let mut value = Vec::new();

    // loop through clust_threshold vector
    for i in 0..clust_threshold.len() {
        // loop through data vector
        for j in 0..data.len() {
            for k in 0..data[j].len() {
                // Check operator equal to or greater than
                if operator == ">=" {
                    if data[j][k] >= clust_threshold[i] {
                        // push x, y, threshold, and value into vector
                        x.push(k as f32);
                        y.push(j as f32);
                        threshold.push(clust_threshold[i]);
                        value.push(data[j][k]);
                    }
                // Check operator equal to or less than
                } else if operator == "<=" {
                    if data[j][k] <= clust_threshold[i] {
                        // push x, y, threshold, and value into vector
                        x.push(k as f32);
                        y.push(j as f32);
                        threshold.push(clust_threshold[i]);
                        value.push(data[j][k]);
                    }
                // Check operator equal to greater
                } else if operator == ">" {
                    if data[j][k] > clust_threshold[i] {
                        // push x, y, threshold, and value into vector
                        x.push(k as f32);
                        y.push(j as f32);
                        threshold.push(clust_threshold[i]);
                        value.push(data[j][k]);
                    }
                // Check operator equal to less
                } else if operator == "<" {
                    if data[j][k] < clust_threshold[i] {
                        // push x, y, threshold, and value into vector
                        x.push(k as f32);
                        y.push(j as f32);
                        threshold.push(clust_threshold[i]);
                        value.push(data[j][k]);
                    }
                }
            }
        }
    }
    // Store x, y, threshold, and value into a vec![]
    let mut thresholded_data = vec![x, y, threshold, value];
    // Return thresholded_data
    return thresholded_data;
}