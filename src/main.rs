use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use tokio_tungstenite::tungstenite::Message;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8010";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server is running on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: tokio::net::TcpStream) {
    if let Ok(mut ws_stream) = accept_async(stream).await {
        println!("New WebSocket connection established");

        // Receive and handle WebSocket messages
        while let Some(Ok(message)) = ws_stream.next().await {
            // Process the received message
            println!("Received message: {:?}", message);

            // Prepare the reply message
            let reply_message = format!("Replying to: {:?}", message);

            // Send the reply message back to the client
            if let Err(e) = ws_stream.send(Message::Text(reply_message)).await {
                eprintln!("Failed to send message: {:?}", e);
                break;
            }
        }

        println!("WebSocket connection closed");
    } else {
        println!("Failed to accept WebSocket connection");
    }
}

