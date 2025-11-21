use clap::Parser; // <-- Import für die CLI-Argumente
use std::net::IpAddr; // For IpAddr Typ
use futures::future::join_all; // For asynchrone Tasks
use ipnet::IpNet; // For CIDR processing

// Import Modules
mod dns; // DNS module
mod scanner; // Scanner module
mod ports; // Port scanning module
use scanner::ping_single_host;


/// CLI Argument Structure
/// --- 1. Definition ---
#[derive(Parser, Debug)] // The Parser derive macro from clap module
#[command(author = "Tphy", version, about = "Fast network reconnaissance tool", long_about = None)]
struct Args {
    /// The target IP or Network CIDR (e.g., --ip 192.168.0.1 or 192.168.0.0/24)
    #[arg(long = "ip", short = 'i')] 
    target: String,

    /// Custom DNS server for name resolution (optional)
    #[arg(long, short, default_value = "192.168.178.1")]
    dns: IpAddr,

    /// Ports to scan. Can be empty (default top 10) or custom list (e.g., --ports 80,443)
    // If no value is provided, default top 10 ports will be scanned.
    #[arg(long, short = 'p', num_args=0..=1, default_missing_value = "default")] // default_missing_value sets value to "default" if -p is provided without a value
    ports: Option<String>,
}

/// --- Main Function ---
/// The main entry point of the application with Tokio runtime
#[tokio::main]
async fn main() {
    // --- 2. Parse CLI arguments ---
    let args = Args::parse(); // CLI argument parsing
    let target_input = args.target.as_str(); // The target IP or CIDR from the arguments
    let dns_server = args.dns; // The DNS IP from the arguments (or the default)
    let ports_to_scan: Vec<u16> = match args.ports.as_deref() { // Determine ports to scan based on CLI input. As_deref converts Option<String> to Option<&str>. &str is lighter weight. Reference to string.
      None => Vec::new(), // No port scanning
      Some("default") => crate::ports::COMMON_PORTS.to_vec(), // only -p -> Top 10 default ports
      Some(custom_list) => {
        //parsing the string "80,443,8080" into a vector of u16
        custom_list.split(',')
          .filter_map(|s| s.trim().parse::<u16>().ok()) // Parse each port, ignore invalid entries
          .collect() // Collect into a vector
      } 
    };
    // parsing done

    // main info output
    println!("\n{:-<100}", ""); // Separator line
    println!("--- Network Scanner ---");
    println!("Target: {}", target_input);
    println!("DNS Server: {}", dns_server);
    if !ports_to_scan.is_empty() {
        println!("Port Scanning enabled. Ports: {:?}", ports_to_scan);
    }


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
        for ip in net.hosts().collect::<Vec<_>>().into_iter().skip(1).rev().skip(1).rev() { // Convert hosts to Vec, iterate over it, skip the first IP, reverse order, skip the last IP, then reverse back. Result: all host IPs except the first and last.
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


    // Info output about number of hosts to scan
    let total_hosts = scan_ips.len(); // Total number of hosts to scan
    if total_hosts == 0 {
        println!("No scannable host addresses found.");
        return;
    }
    println!("Scanning {} target(s)... just count to 5.", total_hosts);

    
    // --- 4. start scan ---
    let mut scan_tasks = vec![]; // Vector to hold scan tasks
    for ip_addr in scan_ips { // For each IP to scan

        let ports_clone = ports_to_scan.clone(); // Clone ports vector because we need to move it into the async block
        // Spawn Tasks for each IP with Port scan option
        let task = tokio::spawn(ping_single_host(ip_addr, dns_server, ports_clone)); // DNS server passed to ping function
        scan_tasks.push(task);
    } 
    // Wait for all tasks to complete
    let results = join_all(scan_tasks).await; // Wait for all tasks to complete


    // --- 5. Output ---
    println!("\n{:-<100}", ""); // Separator line
    println!(
        "{:<3} | {:<16} | {:<8} | {:<37} | {}",  // ":" starts aligment definition - "<" left align - "width"
        "St", "IP-Address", "Time", "Hostname", "Ports"  // Status, IP, Time, Hostname, Ports -> Table Header
    );
    println!("{:-<100}", ""); // Separator line


    // Prepare and print results
    let mut devices_found = 0; // Counter for found devices
    for result in results {
        if let Ok(Some((ip, duration, hostname, ports))) = result { // Check if ping was successful
            
            let ports_display = if ports.is_empty() { // Check if ports vector is empty
                String::new() // No ports to display
            } else {
                format!("{:?}", ports) // "[80, 443]"
            };

            // {:<16} reserves 16 characters for the IP address, left-aligned.
            // {:<37} reserves 37 characters for the hostname, left-aligned.

            println!(
                "✅ | {:<16} | {:<10.2?} | {:<37} | {}",
                ip, duration, hostname, ports_display
            ); // Print results
            devices_found += 1;
        }
    }
    println!("{:-<100}", "");
    println!("Scan completed. {} device(s) found.", devices_found);
}