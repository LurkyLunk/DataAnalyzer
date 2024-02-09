use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a real scenario, replace this with actual data fetching and processing logic
            match socket.read(&mut buf).await {
                Ok(_) => {
                    let data: Value = serde_json::from_slice(&buf).unwrap();
                    println!("Received data: {:?}", data);
                },
                Err(e) => println!("Error reading data: {:?}", e),
            }
        });
    }
}
