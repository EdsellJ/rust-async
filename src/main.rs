use tokio;
use std::thread;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    
    println!("Running with {} threads", num_threads);
    
    let mut handles = vec![];

    // Spawn tasks equal to number of physical threads
    for i in 1..=num_threads {
        let handle = tokio::spawn(async move {
            println!("Thread {} is running", i);
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}
