use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the filename from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <filename> <req_num>", args[0]);
        eprintln!("args: {:?}", args);
        return Ok(());
    }

    let filename = &args[1];

    // Connect to the server
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to the server");

    // Retrieve and print client and server address information
    let client_addr = socket.local_addr()?;
    let server_addr = socket.peer_addr()?;
    println!(
        "Client IP: {}, Port: {}; Server IP: {}, Port: {}",
        client_addr.ip(),
        client_addr.port(),
        server_addr.ip(),
        server_addr.port()
    );

    // Send the filename to the server
    socket.write_all(filename.as_bytes()).await?;
    println!("Sent filename: {} - req_num: {}", filename, args[2]);

    // Wait and read the response from the server
    let mut response = vec![0; 1024];
    let n = socket.read(&mut response).await?;

    // Print the server's response
    if n == 0 {
        println!("Connection closed by server");
    } else {
        let result = String::from_utf8_lossy(&response[..n]);
        println!("Received from server:\n{}", result);
        println!("Request number: {}\n", args[2])
    }

    Ok(())
}
