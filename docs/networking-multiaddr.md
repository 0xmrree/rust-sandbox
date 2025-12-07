# Multiaddr: Composable Network Addresses

A **Multiaddr** (multiaddress) is like a URL, but more flexible and composable. It describes how to reach a network endpoint by stacking protocols together.

Used heavily in **libp2p** (peer-to-peer networking), but the concept is useful for understanding modern network addressing.

## The Problem with Traditional Addresses

### Traditional Address (Limited)

```
192.168.1.1:8080
```

**Problems:**
- ❌ Assumes TCP/IP
- ❌ Doesn't describe the full path
- ❌ Can't express protocol layers
- ❌ Hard to extend

### URL (Better, but Still Limited)

```
http://example.com:80/path
```

**Problems:**
- ❌ Tied to HTTP
- ❌ Can't describe arbitrary protocol stacks
- ❌ Doesn't work well for P2P

## Multiaddr Solution

A Multiaddr **explicitly describes each protocol layer** in the path to a resource.

### Format

```
/protocol/value/protocol/value/...
```

Each segment describes one layer of the network stack.

## Examples

### Basic TCP/IP

**Traditional:**
```
192.168.1.1:8080
```

**Multiaddr:**
```
/ip4/192.168.1.1/tcp/8080
```

**Reading it:** "Use IPv4 to reach 192.168.1.1, then use TCP on port 8080"

### IPv6 with TCP

**Traditional:**
```
[2001:db8::1]:8080
```

**Multiaddr:**
```
/ip6/2001:db8::1/tcp/8080
```

### WebSocket over TCP

**Traditional URL:**
```
ws://example.com:8080/path
```

**Multiaddr:**
```
/dns4/example.com/tcp/8080/ws
```

**Reading it:** "Resolve example.com via DNS (IPv4), connect via TCP on port 8080, then use WebSocket"

### QUIC (UDP-based protocol)

**Multiaddr:**
```
/ip4/127.0.0.1/udp/4001/quic
```

**Reading it:** "IPv4 to 127.0.0.1, UDP on port 4001, then QUIC protocol"

### Complex: WebSocket over TLS over TCP

**Multiaddr:**
```
/ip4/192.168.1.1/tcp/443/tls/ws
```

**Reading it:** "IPv4 → TCP port 443 → TLS encryption → WebSocket"

### P2P with libp2p

**Multiaddr:**
```
/ip4/127.0.0.1/tcp/4001/p2p/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N
```

**Reading it:** "IPv4 → TCP port 4001 → connect to peer with ID QmYyQ..."

## Common Protocol Components

| Protocol | Example | Description |
|----------|---------|-------------|
| `ip4` | `/ip4/192.168.1.1` | IPv4 address |
| `ip6` | `/ip6/2001:db8::1` | IPv6 address |
| `dns4` | `/dns4/example.com` | DNS resolution (IPv4) |
| `dns6` | `/dns6/example.com` | DNS resolution (IPv6) |
| `tcp` | `/tcp/8080` | TCP port |
| `udp` | `/udp/4001` | UDP port |
| `quic` | `/quic` | QUIC protocol |
| `ws` | `/ws` | WebSocket |
| `wss` | `/wss` | WebSocket Secure |
| `tls` | `/tls` | TLS encryption |
| `p2p` | `/p2p/QmHash...` | libp2p peer ID |
| `http` | `/http` | HTTP protocol |
| `https` | `/https` | HTTPS protocol |

## Why Multiaddr?

### 1. **Explicit Protocol Stack**

Traditional addresses hide the protocol stack. Multiaddr makes it explicit.

```
Traditional: example.com:443
Multiaddr:   /dns4/example.com/tcp/443/tls/http
```

You know exactly what protocols are being used!

### 2. **Composable**

You can build addresses by stacking protocols:

```rust
// Pseudocode
let base = "/ip4/127.0.0.1/tcp/8080";
let with_ws = base + "/ws";
let with_tls = base + "/tls/ws";
```

### 3. **Future-Proof**

New protocols can be added without breaking existing code:

```
/ip4/127.0.0.1/tcp/8080/newprotocol/value
```

### 4. **Transport Agnostic**

Works with any transport layer:

```
/ip4/127.0.0.1/tcp/8080      (TCP)
/ip4/127.0.0.1/udp/8080      (UDP)
/ip4/127.0.0.1/udp/8080/quic (QUIC over UDP)
```

### 5. **Perfect for P2P**

Describes complex P2P connection paths:

```
/ip4/192.168.1.1/tcp/4001/p2p/QmPeerID/p2p-circuit/p2p/QmRelayID
```

This describes: "Connect to peer QmPeerID via relay QmRelayID"

## Rust Example with libp2p

```rust
use libp2p::Multiaddr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a multiaddr
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse()?;
    println!("Address: {}", addr);
    
    // Build a multiaddr programmatically
    let mut addr = Multiaddr::empty();
    addr.push(Protocol::Ip4([127, 0, 0, 1].into()));
    addr.push(Protocol::Tcp(8080));
    addr.push(Protocol::Ws("/".into()));
    println!("Built address: {}", addr);
    // Output: /ip4/127.0.0.1/tcp/8080/ws
    
    // Iterate over protocols
    for protocol in addr.iter() {
        println!("Protocol: {:?}", protocol);
    }
    
    Ok(())
}
```

## Parsing Multiaddrs

```rust
use libp2p::{Multiaddr, multiaddr::Protocol};

fn parse_example() -> Result<(), Box<dyn std::error::Error>> {
    let addr: Multiaddr = "/ip4/192.168.1.1/tcp/8080/ws".parse()?;
    
    // Extract components
    for component in addr.iter() {
        match component {
            Protocol::Ip4(ip) => println!("IPv4: {}", ip),
            Protocol::Tcp(port) => println!("TCP port: {}", port),
            Protocol::Ws(_) => println!("WebSocket enabled"),
            _ => println!("Other protocol: {:?}", component),
        }
    }
    
    Ok(())
}
```

## Real-World Examples

### HTTP Server

**Traditional:**
```
http://localhost:8080
```

**Multiaddr:**
```
/ip4/127.0.0.1/tcp/8080/http
```

### HTTPS Server

**Traditional:**
```
https://example.com:443
```

**Multiaddr:**
```
/dns4/example.com/tcp/443/tls/http
```

### WebSocket Server

**Traditional:**
```
ws://localhost:9000
```

**Multiaddr:**
```
/ip4/127.0.0.1/tcp/9000/ws
```

### IPFS Node

**Multiaddr:**
```
/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ
```

**Reading it:** "Connect to 104.131.131.82 on TCP port 4001, then connect to IPFS peer QmaC..."

### WebRTC Connection

**Multiaddr:**
```
/ip4/192.168.1.1/udp/9090/webrtc/p2p/QmPeerID
```

## Comparison

| Aspect | Traditional | URL | Multiaddr |
|--------|-------------|-----|-----------|
| Format | `192.168.1.1:8080` | `http://example.com:80` | `/ip4/192.168.1.1/tcp/8080` |
| Explicit protocols | ❌ | Partial | ✅ |
| Composable | ❌ | ❌ | ✅ |
| Transport agnostic | ❌ | ❌ | ✅ |
| P2P friendly | ❌ | ❌ | ✅ |
| Future-proof | ❌ | Partial | ✅ |

## When to Use Multiaddr

### ✅ Good For:
- **P2P applications** (libp2p, IPFS)
- **Protocol-agnostic systems**
- **Complex network stacks**
- **Future-proof addressing**

### ❌ Overkill For:
- **Simple client-server apps**
- **Traditional web services**
- **When everyone uses HTTP/HTTPS**

## Converting Between Formats

### Traditional → Multiaddr

```rust
// Traditional
let traditional = "192.168.1.1:8080";

// Convert to Multiaddr
let multiaddr = format!("/ip4/{}/tcp/{}", 
    "192.168.1.1", 
    "8080"
).parse::<Multiaddr>()?;
```

### URL → Multiaddr

```rust
// URL
let url = "https://example.com:443/path";

// Convert to Multiaddr (conceptually)
let multiaddr = "/dns4/example.com/tcp/443/tls/http".parse::<Multiaddr>()?;
```

## Summary

**Multiaddr** is a self-describing network address format that:

1. **Explicitly lists all protocol layers** - no assumptions
2. **Is composable** - build complex addresses by stacking protocols
3. **Is future-proof** - new protocols can be added easily
4. **Works great for P2P** - describes complex connection paths
5. **Is human-readable** - you can see exactly what's happening

### Quick Examples

```
/ip4/127.0.0.1/tcp/8080                          (Basic TCP)
/ip6/::1/tcp/8080                                (IPv6 TCP)
/dns4/example.com/tcp/443/tls/http               (HTTPS)
/ip4/192.168.1.1/tcp/4001/p2p/QmPeerID           (P2P)
/ip4/127.0.0.1/udp/9090/quic                     (QUIC)
/dns4/example.com/tcp/443/wss                    (WebSocket Secure)
```

Think of it as a **URL on steroids** - it can describe any network path, not just HTTP!
