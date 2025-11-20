use clap::Parser; // <-- Import für die CLI-Argumente
use std::net::IpAddr; // For IpAddr Typ
use futures::future::join_all; // For asynchrone Tasks
use ipnet::IpNet; // For CIDR processing

// Import Modules
mod dns;
mod scanner;
use scanner::ping_single_host;

// --- 1. Definition ---
#[derive(Parser, Debug)]
#[command(author = "Tphy", version, about = "Fast network reconnaissance tool", long_about = None)]
struct Args {
    /// The target IP or Network CIDR (e.g., --ip 192.168.0.1 or 192.168.0.0/24)
    #[arg(long = "ip", short = 'i')] 
    target: String,

    /// Custom DNS server for name resolution (optional)
    #[arg(long, short, default_value = "192.168.178.1")]
    dns: IpAddr,
}

#[tokio::main]
async fn main() {
    // --- 2. Parse CLI arguments ---
    let args = Args::parse(); // CLI argument parsing

    let target_input = args.target.as_str(); // The target IP or CIDR from the arguments
    let dns_server = args.dns; // The DNS IP from the arguments (or the default)
    println!("--- Network Scanner ---");
    println!("Target: {}", target_input);
    println!("DNS Server: {}", dns_server);

    // --- 3. Process target (CIDR or Single IP) ---
    let is_cidr = target_input.contains('/'); // Check if CIDR notation is used
    let mut scan_ips: Vec<IpAddr> = Vec::new(); // Vector to hold IPs to scan

    if is_cidr { 
        // CIDR Logik
        let net: IpNet = match target_input.parse() { // Parse CIDR
            Ok(n) => n, // If successful, use the network
            Err(_) => { // If parsing fails, print error and exit
                eprintln!("Error: '{}' is not a valid CIDR address.", target_input);
                return;
            }
        };
        
        // Collect all usable host IPs in the CIDR range, excluding network and broadcast addresses
        for ip in net.hosts().collect::<Vec<_>>().into_iter().skip(1).rev().skip(1).rev() { // Skip network and broadcast
             scan_ips.push(ip); // Add to vector
        }
    } else {
        // Single IP Logic
        match target_input.parse::<IpAddr>() { 
            Ok(ip) => scan_ips.push(ip), // If successful, add to vector
            Err(_) => { // If parsing fails, print error and exit
                eprintln!("Error: '{}' is neither a valid CIDR nor IP.", target_input); 
                return;
            }
        }
    }

    let total_hosts = scan_ips.len(); // Total number of hosts to scan
    if total_hosts == 0 {
        println!("No scannable host addresses found.");
        return;
    }

    println!("Scanning {} target(s)... Please wait.", total_hosts);

    // --- 4. start scan ---
    let mut scan_tasks = vec![];
    
    for ip_addr in scan_ips {
        // Spawn Tasks for each IP
        let task = tokio::spawn(ping_single_host(ip_addr, dns_server)); // DNS server passed to ping function
        scan_tasks.push(task);
    } 

    // Wait for all tasks to complete
    let results = join_all(scan_tasks).await; // Wait for all tasks to complete

    // --- 5. Output ---
    println!("\n--- Scan Results ---");
    let mut devices_found = 0;

    for result in results {
        if let Ok(Some((ip, duration, hostname))) = result { // Check if ping was successful
            println!("✅ Reachable: {:<15} ({}) \tTime: {:.2?}", ip, hostname, duration); // Print reachable IPs with hostname and time
            devices_found += 1;
        }
    }
    println!("Scan completed. {} device(s) found.", devices_found);
}