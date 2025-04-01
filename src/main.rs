use tokio;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    // Spawn 20 tasks
    for i in 1..=20 {
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
