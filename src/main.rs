use std::io;
use std::net::{IpAddr};
use std::time::Duration;
use futures::future::join_all;
use ipnet::IpNet;

// ----------------------------------------------------------------------
// FUNCTION TO HANDLE SINGLE IP PING (Extracted logic for reuse)
// ----------------------------------------------------------------------

async fn ping_single_host(ip_addr: IpAddr) -> Option<(IpAddr, Duration)> { // Returns Option<(IP, Time)> or None (if unreachable). Forces caller to check for errors. Anti NULL protection via RUST.
    // surge_ping::ping is used here
    let result = surge_ping::ping(ip_addr, &[0; 8]).await;
    
    match result {
        Ok((_, duration)) => Some((ip_addr, duration)),
        Err(_) => None, 
    }
}

// ----------------------------------------------------------------------
// MAIN APPLICATION LOGIC
// ----------------------------------------------------------------------
#[tokio::main]
async fn main() {
    println!("--- Multi-Mode Network Scanner ---");
    println!("Enter target(s): [CIDR e.g., 192.168.1.0/24] OR [Single IP e.g., 192.168.1.1]:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let target = input.trim();

    // 1. Determine if the target is a CIDR block or a single IP
    
    let is_cidr = target.contains('/');
    let mut scan_ips: Vec<IpAddr> = Vec::new();

    if is_cidr {
        // --- CIDR LOGIC ---
        let net: IpNet = match target.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: '{}' is not a valid CIDR address.", target);
                return;
            }
        };
        
        // Exclude Network ID (first) and Broadcast Address (last)
        for ip in net.hosts().collect::<Vec<_>>().into_iter().skip(1).rev().skip(1).rev() { // Skip first and last
             scan_ips.push(ip);
        }
        
    } else {
        // --- SINGLE IP LOGIC ---
        // Try to parse the input as a single IP
        match target.parse::<IpAddr>() { // Try to parse as single IP
            Ok(ip) => {
                scan_ips.push(ip);
            },
            Err(_) => {
                eprintln!("Error: '{}' is neither a valid CIDR address nor a single IP address.", target); 
                return;
            }
        }
    }

    let total_hosts = scan_ips.len(); // Total number of hosts to scan
    if total_hosts == 0 {
        println!("No scannable host addresses found.");
        return;
    }

    println!("Scanning {} target(s) in {} mode. Please wait...", total_hosts, 
             if is_cidr { "CIDR" } else { "Single Host" });

    // 2. Create asynchronous tasks
    let mut scan_tasks = vec![];
    
    for ip_addr in scan_ips {
        // We now call the dedicated async function for the ping task
        let task = tokio::spawn(ping_single_host(ip_addr));
        scan_tasks.push(task);
    } 

    // 3. Wait for all tasks in parallel
    let results = join_all(scan_tasks).await;

    // 4. Output results
    println!("\n--- Scan Results for {} ---", target);
    let mut devices_found = 0;

    for result in results {
        // The result here is Option<T> because the task handles the error internally
        if let Ok(Some((ip, duration))) = result {
            println!("✅ Reachable: {} (Time: {:.2?})", ip, duration);
            devices_found += 1;
        }
    }
    println!("Scan completed. {} device(s) found.", devices_found);
}