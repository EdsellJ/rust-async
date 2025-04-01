use tokio::sync::Semaphore;
use tokio::task;
use reqwest;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com",
        "https://www.rust-lang.org",
        "https://www.wikipedia.org",
        "https://www.github.com",
    ];

    let semaphore = Arc::new(Semaphore::new(2)); // Allow max 2 concurrent tasks

    let mut handles = vec![];

    for url in urls {
        let permit = semaphore.clone().acquire_owned().await.unwrap(); // Acquire a permit
        let handle = task::spawn(async move {
            let _permit = permit; // Keep it in scope to avoid early drop
            match fetch_page(url).await {
                Ok(content) => println!("✅ Fetched {} ({} bytes)", url, content.len()),
                Err(err) => println!("❌ Failed {}: {}", url, err),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn fetch_page(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

