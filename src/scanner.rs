use std::net::IpAddr;
use std::time::Duration;
use crate::dns::reverse_lookup; 
use crate::ports::scan_ports;

/// Ping a single host and return its IP, response time, and hostname if reachable.
/// The DNS server IP is passed to perform reverse DNS lookup.
pub async fn ping_single_host(ip_addr: IpAddr, dns_server: IpAddr, ports_to_scan: Vec<u16>) -> Option<(IpAddr, Duration, String, Vec<u16>)> { // Ping the IP address and return relevant info if reachable
    
    let result = surge_ping::ping(ip_addr, &[0; 8]).await; // Ping the IP address. We use 8 bytes of data. 
    // In short: 8 bytes are used because they are sufficient to test the functionality of the connection and measure latency, while also being very efficient for scanning purposes.
    
    
    match result {
        Ok((_, duration)) => {
            // 1. If Host is reachable, perform reverse DNS lookup
            let hostname = reverse_lookup(ip_addr, dns_server).await;
            
            // 2. If port scanning is enabled, scan common ports
            let open_ports = if !ports_to_scan.is_empty() {
                scan_ports(ip_addr, ports_to_scan).await
            } else {
                Vec::new() // Return empty vector if port scanning is disabled
            };

            // 3. Return the results
            Some((ip_addr, duration, hostname, open_ports)) // Return IP, response time, hostname and open ports
        },
        Err(_) => None, 
    }
}