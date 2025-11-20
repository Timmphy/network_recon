use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts, NameServerConfig, Protocol};

/// Trys to perform a reverse DNS lookup for the given IP address using the specified DNS server.
/// If the lookup is successful, it returns the hostname; otherwise, it returns the IP address as a string.
pub async fn reverse_lookup(ip_addr: IpAddr, dns_server: IpAddr) -> String {
    
    // 1. Configuration using the provided DNS server
    let mut config = ResolverConfig::new();
    
    // We create a SocketAddr for the DNS server
    let socket_addr = SocketAddr::new(dns_server, 53); //socket address with DNS server IP and port 53. A SocketAddr combines an IP address and a port number.
        
    config.add_name_server(NameServerConfig { // Add the custom DNS server
        socket_addr, 
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    });

    // 2. Set Resolver Options
    let mut opts = ResolverOpts::default(); //must be mutable to change timeout and attempts
    opts.timeout = Duration::from_secs(3); // Set timeout to 3 seconds
    opts.attempts = 1; // Set number of attempts to 1

    // 3. Create the resolver
    let resolver = TokioAsyncResolver::tokio(config, opts); // Create the async resolver. TokioAsyncResolver is used for asynchronous DNS resolution in a Tokio runtime.

    // 4. Request reverse lookup
    let lookup_result = resolver.reverse_lookup(ip_addr).await; // Perform the reverse DNS lookup asynchronously

    match lookup_result {
        Ok(ptr_response) => {
            if let Some(name) = ptr_response.iter().next() { // Get the first PTR record. A PTR record maps an IP address to a hostname.
                return name.to_string().trim_end_matches('.').to_string(); // Return the hostname without trailing dot. Trailing dot is common in DNS names to indicate the root.
            }
            ip_addr.to_string()
        },
        Err(_) => ip_addr.to_string(), 
    }
}