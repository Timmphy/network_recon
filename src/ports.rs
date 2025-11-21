use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

// List of common ports to scan
pub const COMMON_PORTS: [u16; 10] = [ // u16 and not u8 because ports go up to 65535
    21, 22, 23, 53, 80, 110, 143, 443, 3306, 8080
];


/// scan a list of common ports on the given IP address
/// Use ports_to_scan to specify which ports to scan
pub async fn scan_ports(ip: IpAddr, ports_to_scan: Vec<u16>) -> Vec<u16> { // Vector to hold open ports
    let mut open_ports = Vec::new(); // Iterate over common ports

    for port in ports_to_scan { // For each port in the list 
        if is_port_open(ip, port).await { // If open, add to vector
            open_ports.push(port); // Add open port to the list
        }
    }
    open_ports // Return the list of open ports
}

/// Check if a specific port on the given IP address is open
async fn is_port_open(ip: IpAddr, port: u16) -> bool { // create a socket address from the IP and port and check if it's open. Return true if open, false otherwise.
    let socket_addr = SocketAddr::new(ip, port);
    let timeout_duration = Duration::from_millis(800); // 800ms Timeout. Firewall can cause delays from dropping packets.

    // We try to connect to the socket address with a timeout
    match timeout(timeout_duration, TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_stream)) => true, // Successful connection -> Port open
        _ => false,              // Timeout or error -> Port closed. // _=> means we don't care about the specific error
    }
}