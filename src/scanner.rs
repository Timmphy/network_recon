use std::net::IpAddr;
use std::time::Duration;
use crate::dns::reverse_lookup; 

/// Ping a single host and return its IP, response time, and hostname if reachable.
/// The DNS server IP is passed to perform reverse DNS lookup.
pub async fn ping_single_host(ip_addr: IpAddr, dns_server: IpAddr) -> Option<(IpAddr, Duration, String)> { // DNS server IP added as parameter
    
    let result = surge_ping::ping(ip_addr, &[0; 8]).await; // Ping the IP address. We use 8 bytes of data. 
    // In short: 8 bytes are used because they are sufficient to test the functionality of the connection and measure latency, while also being very efficient for scanning purposes.
    match result {
        Ok((_, duration)) => {
            // Host is reachable, perform reverse DNS lookup
            let hostname = reverse_lookup(ip_addr, dns_server).await;
            
            Some((ip_addr, duration, hostname)) // Return IP, response time, and hostname
        },
        Err(_) => None, 
    }
}