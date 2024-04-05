use std::thread;

fn main() {
    // Define the number of threads to be used
    let num_threads = 4;
    
    // Divide the task into equal parts for each thread
    let chunk_size = 100 / num_threads;
    
    // Vector to store thread handles
    let mut handles = vec![];

    // Loop to create and execute the threads
    for i in 0..num_threads {
        let start = i * chunk_size + 1;
        let end = start + chunk_size;
        
        // Clone variables to be moved into the thread
        let handle = thread::spawn(move || {
            let mut sum = 0;
            for j in start..end {
                sum += j;
            }
            sum
        });
        
        // Add the thread handle to the vector
        handles.push(handle);
    }

    // Loop to collect results from each thread
    let mut total_sum = 0;
    for handle in handles {
        // Wait for each thread to complete and sum their results
        total_sum += handle.join().unwrap();
    }

    // Print the final result
    println!("Total sum: {}", total_sum);
}
