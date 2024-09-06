use std::io::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the TCP listener to a specific address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        // Accept an incoming TCP connection
        let (socket, addr) = listener.accept().await?;

        println!("Accepted connection from: {}", addr);

        // Spawn a new task to handle each connection concurrently
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {:?}", e);
            }
        });
    }
}

async fn handle_client(
    mut socket: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    // Read the filename from the client
    let n = socket.read(&mut buf).await?;

    if n == 0 {
        return Ok(());
    }

    let filename = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    println!("Received request for file: {}", filename);

    // Read the file and calculate statistics asynchronously
    match calculate_file_stats(&filename).await {
        Ok(stats) => {
            // Send the results back to the client
            let response = format!(
                "File: {}\nLines: {}\nWords: {}\n",
                filename, stats.0, stats.1
            );
            socket.write_all(response.as_bytes()).await?;
        }
        Err(e) => {
            let response = format!("Error reading file: {}\n", e);
            socket.write_all(response.as_bytes()).await?;
        }
    }

    Ok(())
}

async fn calculate_file_stats(filename: &str) -> Result<(usize, usize), Error> {
    let file = File::open(filename).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut line_count = 0;
    let mut word_count = 0;

    while let Some(line) = lines.next_line().await? {
        line_count += 1;
        word_count += line.split_whitespace().count();
    }

    Ok((line_count, word_count))
}
