#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("Hello, world!");
    });
    handle.await.unwrap();
}

