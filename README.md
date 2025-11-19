# 🚀 Network Reconnaissance Scanner (Net-Recon)

A high-performance, asynchronous network scanner written in **Rust**. This tool allows for fast ICMP (Ping) scanning of entire CIDR subnets or checking the reachability of a single IP address using the `tokio` runtime.

## ✨ Features

* **Multi-Mode Scanning:** Supports scanning of both **CIDR** network blocks (e.g., `192.168.1.0/24`) and **Single IP** addresses (e.g., `10.0.0.5`).
* **Asynchronous Performance:** Utilizes the `tokio` runtime and `futures::join_all` to concurrently ping multiple hosts, significantly reducing scan time.
* **Host Exclusion:** Automatically excludes the **Network ID** (first address) and **Broadcast Address** (last address) from CIDR scans using the `ipnet` crate.
* **Clear Output:** Displays reachable hosts (`✅ Reachable`) along with their round-trip time (latency).

---

## 🛠️ Prerequisites

To compile and run this scanner, you must have the following installed:

1.  **Rust and Cargo:** Install the Rust toolchain via `rustup`.
2.  **Required Privileges:** Since the tool performs low-level network operations (ICMP pinging), it must be run with **elevated privileges** (`sudo` on Linux, Administrator on Windows).

---

## 📦 Building and Running

The project is built using Cargo. **The process differs slightly between Linux and Windows due to platform-specific network access permissions.**

### 1. Clone the Repository

First, clone the project and navigate into the directory:

```bash
git clone https://github.com/Timmphy/network_recon.git
cd network_recon
```
### 2. Compile and Run (Platform-Specific)
## Linux / WSL (Recommended for Development)
On Linux, elevated privileges (sudo) are strictly required to send ICMP packets.
```bash
# 1. Compile the optimized release binary
cargo build --release

# 2. Run the scanner with sudo
sudo ./target/release/network_recon
```
## Windows
On Windows, the compiled executable (.exe) should be run with Administrator privileges.
```bash
# 1. Compile the optimized release executable
cargo build --release

# 2. Navigate to the release folder and execute:
# Note: You MUST right-click the file and select "Run as administrator" or run from an Admin PowerShell/CMD.
.\target\release\network_recon.exe
```