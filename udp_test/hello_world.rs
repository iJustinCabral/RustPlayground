use std::net::UdpSocket;
use std::net::SocketAddr;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
        let addr: SocketAddr = "127.0.0.1:34255".parse().expect("Failed to parse");
        let message = b"Hello, Netcat!";

        socket.send_to(message, addr).expect("Failed to send message");
        
    }
    Ok(())
}

// Use: nc -l -u 127.0.0.1 34255 to listen to the message


