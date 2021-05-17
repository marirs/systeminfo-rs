use std::net::UdpSocket;

/// Get the local ip address, return an `Option<String>`. when it fail, return `None`.
pub fn get_local_ip() -> Option<String> {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| {
            socket.connect("8.8.8.8:80")?;
            socket.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .ok()
}
