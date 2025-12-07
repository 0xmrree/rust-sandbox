# IPv4, IPv6, and Dual-Stack Networking

A practical guide to understanding IP addresses and how computers listen for connections.

## The Basics

### IPv4 (Internet Protocol version 4)

**Format:** Four numbers (0-255) separated by dots

```
192.168.1.1
127.0.0.1 (localhost)
0.0.0.0 (all interfaces)
```

- **32 bits** (4 bytes)
- **~4.3 billion addresses** (running out!)
- Been around since 1983

### IPv6 (Internet Protocol version 6)

**Format:** Eight groups of hexadecimal numbers separated by colons

```
2001:0db8:85a3:0000:0000:8a2e:0370:7334
::1 (localhost - shorthand for 0000:0000:0000:0000:0000:0000:0000:0001)
:: (all interfaces - shorthand for all zeros)
```

- **128 bits** (16 bytes)
- **340 undecillion addresses** (we'll never run out!)
- Designed to replace IPv4

### Why Both Exist

- **IPv4** - Old but everywhere, running out of addresses
- **IPv6** - New, plenty of addresses, but not everywhere yet
- **Transition period** - We're living in it! Both protocols coexist

## Special Addresses

### IPv4 Special Addresses

| Address | Meaning |
|---------|---------|
| `127.0.0.1` | Localhost (this computer) |
| `0.0.0.0` | All interfaces (listen on all) |
| `192.168.x.x` | Private network (home/office) |
| `10.x.x.x` | Private network |

### IPv6 Special Addresses

| Address | Meaning |
|---------|---------|
| `::1` | Localhost (this computer) |
| `::` | All interfaces (listen on all) |
| `fe80::` | Link-local (local network only) |
| `::ffff:192.168.1.1` | IPv4-mapped IPv6 address |

## IPv4-Mapped IPv6 Addresses

This is the confusing part! An IPv4 address can be represented as IPv6:

```
IPv4:  192.168.1.1
IPv6:  ::ffff:192.168.1.1  (IPv4-mapped IPv6)
```

This allows IPv6 sockets to handle IPv4 connections.

## Listening on Ports

When you listen on a port, you need to choose:
1. **IPv4 only** - `0.0.0.0:8080`
2. **IPv6 only** - `[::]:8080`
3. **Dual-stack** - Listen on both (OS-dependent behavior)

### How the OS Decides

When you bind to a socket:

```rust
// IPv4 only
let listener = TcpListener::bind("0.0.0.0:8080")?;

// IPv6 only
let listener = TcpListener::bind("[::]:8080")?;
```

**Key insight:** On most systems, binding to `[::]` (IPv6 all interfaces) will ALSO accept IPv4 connections via IPv4-mapped addresses!

## Rust Examples

### Example 1: IPv4 Only Server

```rust
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // Listen on IPv4 only
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Listening on IPv4: 0.0.0.0:8080");
    
    for stream in listener.incoming() {
        let stream = stream?;
        let peer = stream.peer_addr()?;
        println!("Connection from: {}", peer);
        // peer will be IPv4: 192.168.1.5:54321
    }
    
    Ok(())
}
```

**What can connect:**
- ✅ IPv4 clients: `192.168.1.5`
- ❌ IPv6 clients: `2001:db8::1` (won't work)

### Example 2: IPv6 Only Server (Strict)

```rust
use std::net::{TcpListener, SocketAddr};

fn main() -> std::io::Result<()> {
    // Listen on IPv6 only, disable IPv4-mapped addresses
    let listener = TcpListener::bind("[::]:8080")?;
    
    // On Linux, you'd set IPV6_V6ONLY socket option to true
    // to disable IPv4-mapped addresses (more complex)
    
    println!("Listening on IPv6: [::]:8080");
    
    for stream in listener.incoming() {
        let stream = stream?;
        let peer = stream.peer_addr()?;
        println!("Connection from: {}", peer);
        // peer will be IPv6: [2001:db8::1]:54321
    }
    
    Ok(())
}
```

**What can connect:**
- ❌ IPv4 clients: `192.168.1.5` (won't work if IPV6_V6ONLY is set)
- ✅ IPv6 clients: `2001:db8::1`

### Example 3: Dual-Stack Server (Default Behavior)

```rust
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // Listen on IPv6, which usually accepts IPv4 too!
    let listener = TcpListener::bind("[::]:8080")?;
    println!("Listening on dual-stack: [::]:8080");
    
    for stream in listener.incoming() {
        let stream = stream?;
        let peer = stream.peer_addr()?;
        
        match peer {
            std::net::SocketAddr::V4(addr) => {
                println!("IPv4 connection from: {}", addr);
            }
            std::net::SocketAddr::V6(addr) => {
                // Might be a real IPv6 or IPv4-mapped
                if addr.ip().to_ipv4_mapped().is_some() {
                    println!("IPv4-mapped connection from: {}", addr);
                } else {
                    println!("IPv6 connection from: {}", addr);
                }
            }
        }
    }
    
    Ok(())
}
```

**What can connect:**
- ✅ IPv4 clients: `192.168.1.5` (appears as `::ffff:192.168.1.5`)
- ✅ IPv6 clients: `2001:db8::1`

### Example 4: Explicit Dual-Stack (Two Listeners)

```rust
use std::net::TcpListener;
use std::thread;

fn main() -> std::io::Result<()> {
    // Explicitly listen on both IPv4 and IPv6
    
    // IPv4 listener
    let listener_v4 = TcpListener::bind("0.0.0.0:8080")?;
    thread::spawn(move || {
        println!("IPv4 listener on 0.0.0.0:8080");
        for stream in listener_v4.incoming() {
            if let Ok(stream) = stream {
                println!("IPv4 connection: {}", stream.peer_addr().unwrap());
            }
        }
    });
    
    // IPv6 listener
    let listener_v6 = TcpListener::bind("[::]:8080")?;
    println!("IPv6 listener on [::]:8080");
    for stream in listener_v6.incoming() {
        if let Ok(stream) = stream {
            println!("IPv6 connection: {}", stream.peer_addr().unwrap());
        }
    }
    
    Ok(())
}
```

**Note:** This might fail if the OS doesn't allow binding the same port twice!

### Example 5: Checking Address Type

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let ipv4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let ipv6 = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    let ipv4_mapped = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc0a8, 0x0101));
    
    println!("IPv4: {}", ipv4);
    println!("Is IPv4? {}", ipv4.is_ipv4());
    
    println!("\nIPv6: {}", ipv6);
    println!("Is IPv6? {}", ipv6.is_ipv6());
    
    println!("\nIPv4-mapped: {}", ipv4_mapped);
    if let IpAddr::V6(v6) = ipv4_mapped {
        if let Some(v4) = v6.to_ipv4_mapped() {
            println!("This is IPv4-mapped! Original IPv4: {}", v4);
        }
    }
}
```

**Output:**
```
IPv4: 192.168.1.1
Is IPv4? true

IPv6: 2001:db8::1
Is IPv6? true

IPv4-mapped: ::ffff:192.168.1.1
This is IPv4-mapped! Original IPv4: 192.168.1.1
```

### Example 6: Connecting as a Client

```rust
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect using IPv4
    let stream_v4 = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected via IPv4");
    
    // Connect using IPv6
    let stream_v6 = TcpStream::connect("[::1]:8080")?;
    println!("Connected via IPv6");
    
    // Connect using hostname (OS decides IPv4 or IPv6)
    let stream = TcpStream::connect("localhost:8080")?;
    println!("Connected via: {}", stream.peer_addr()?);
    
    Ok(())
}
```

## How DNS Resolution Works

When you connect to a hostname:

```rust
use std::net::ToSocketAddrs;

fn main() -> std::io::Result<()> {
    // Resolve "localhost" - might return both IPv4 and IPv6
    let addrs = "localhost:8080".to_socket_addrs()?;
    
    for addr in addrs {
        println!("Resolved address: {}", addr);
    }
    
    // Output might be:
    // Resolved address: 127.0.0.1:8080 (IPv4)
    // Resolved address: [::1]:8080 (IPv6)
    
    Ok(())
}
```

The OS tries addresses in order until one works!

## Operating System Behavior

### Linux
- Binding to `[::]` accepts both IPv4 and IPv6 by default
- Can set `IPV6_V6ONLY` socket option to disable IPv4-mapped addresses

### Windows
- Binding to `[::]` accepts both IPv4 and IPv6 by default
- Similar socket options available

### macOS
- Binding to `[::]` accepts both IPv4 and IPv6 by default
- Similar behavior to Linux

## Practical Recommendations

### For Servers

**Option 1: Simple Dual-Stack (Recommended)**
```rust
// This usually works for both IPv4 and IPv6
let listener = TcpListener::bind("[::]:8080")?;
```

**Option 2: Explicit IPv4 Only**
```rust
// If you only want IPv4
let listener = TcpListener::bind("0.0.0.0:8080")?;
```

**Option 3: Explicit Dual-Stack**
```rust
// If you need fine control, bind both explicitly
let listener_v4 = TcpListener::bind("0.0.0.0:8080")?;
let listener_v6 = TcpListener::bind("[::]:8081")?; // Different port!
```

### For Clients

**Option 1: Let DNS Decide**
```rust
// Connect to hostname - OS picks IPv4 or IPv6
TcpStream::connect("example.com:80")?;
```

**Option 2: Explicit IPv4**
```rust
TcpStream::connect("93.184.216.34:80")?;
```

**Option 3: Explicit IPv6**
```rust
TcpStream::connect("[2606:2800:220:1:248:1893:25c8:1946]:80")?;
```

## Common Pitfalls

### 1. Forgetting Brackets for IPv6

```rust
// ❌ Wrong - ambiguous with port
TcpListener::bind("::1:8080")?;

// ✅ Correct - brackets disambiguate
TcpListener::bind("[::1]:8080")?;
```

### 2. Assuming IPv6 Only

```rust
// This might accept IPv4 connections too!
let listener = TcpListener::bind("[::]:8080")?;
// IPv4 clients can connect via IPv4-mapped addresses
```

### 3. Hardcoding IP Addresses

```rust
// ❌ Bad - only works on IPv4 networks
TcpStream::connect("192.168.1.1:8080")?;

// ✅ Better - works on both
TcpStream::connect("myserver.local:8080")?;
```

## Testing Your Server

### Test IPv4 Connection
```bash
# Using netcat
nc 127.0.0.1 8080

# Using curl
curl http://127.0.0.1:8080
```

### Test IPv6 Connection
```bash
# Using netcat
nc ::1 8080

# Using curl (note the brackets!)
curl http://[::1]:8080
```

### Check What's Listening
```bash
# Linux/macOS
netstat -tuln | grep 8080

# Or use ss on Linux
ss -tuln | grep 8080

# Output shows:
# tcp   0.0.0.0:8080    (IPv4)
# tcp6  :::8080         (IPv6)
```

## Summary

### Key Concepts

1. **IPv4** - 32-bit addresses (192.168.1.1), running out
2. **IPv6** - 128-bit addresses (2001:db8::1), plenty available
3. **IPv4-mapped IPv6** - IPv4 addresses in IPv6 format (::ffff:192.168.1.1)
4. **Dual-stack** - Supporting both IPv4 and IPv6 simultaneously

### Quick Reference

| Bind Address | IPv4? | IPv6? | Notes |
|--------------|-------|-------|-------|
| `0.0.0.0:8080` | ✅ | ❌ | IPv4 only |
| `[::]:8080` | ✅* | ✅ | Usually dual-stack |
| `127.0.0.1:8080` | ✅ | ❌ | IPv4 localhost |
| `[::1]:8080` | ❌ | ✅ | IPv6 localhost |

*Depends on OS and socket options

### Best Practice

For most applications, bind to `[::]:port` - this gives you dual-stack support and works for both IPv4 and IPv6 clients on most systems!

```rust
// Simple and works for most cases
let listener = TcpListener::bind("[::]:8080")?;
```
