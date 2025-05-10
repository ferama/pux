# 🧵 Pux — Protocol Multiplexer

**Pux** is a lightweight, asynchronous TCP protocol multiplexer written in Rust. It listens on a single port and forwards incoming connections to protocol-specific backend services by detecting the protocol in real time.

Supports:
- 🧠 Protocol detection: HTTP, HTTPS (via TLS), SSH, RDP
- 🚀 Non-blocking, concurrent handling with `tokio`
- 🔍 Deep protocol inspection (no reliance on port numbers)
- 📦 Easy to configure with command-line arguments
- 🧾 Logging via `tracing`

## 🔧 How It Works

When a client connects to `pux`, the multiplexer:
1. Peeks into the first packet sent by the client.
2. Detects the protocol using pattern-based and TLS-based heuristics.
3. Forwards the connection (including the already-received bytes) to the corresponding backend.
4. Pipes traffic bidirectionally for the life of the connection.

```
                          ┌─────────────┐
                          │   Client    │
                          └─────┬───────┘
                                │
                      TCP Connection to Port 5500 (or any, 443?)
                                │
                                ▼
                         ┌───────────────┐
                         │    Pux        │
                         │ (Multiplexer) |
                         └─────┬─────────┘
 ┌──────────────┬──────────────┬──────────────┬──────────────┐
 │              │              │              │              │
 ▼              ▼              ▼              ▼              ▼
SSH           HTTPS           HTTP           RDP         Unknown
"SSH-"       TLS Hello      GET/POST...    RDP SYN       Fallback
banner       w/ SNI          Methods       Packet

 │              │              │              │
 ▼              ▼              ▼              ▼
127.0.0.1:22  127.0.0.1:443  127.0.0.1:80  192.168.1.2:3389

  SSH         HTTPS         HTTP         RDP
 Server       Server       Server       Server

```

## 🏁 Quick Start

### Build

```bash
cargo build --release
```

### Run

```bash
./pux \
    --listen 0.0.0.0:9999 \
    --http 127.0.0.1:8080 \
    --https 127.0.0.1:8443 \
    --ssh 127.0.0.1:2222 \
    --rdp 127.0.0.1:3389
```

Only the protocols you specify will be enabled. At least one backend is required.

## 🛠 CLI Options

| Option     | Description                        | Required |
|------------|------------------------------------|----------|
| `--listen` | Address and port to listen on      | ✅ Yes   |
| `--http`   | Backend for HTTP clients           | Optional |
| `--https`  | Backend for HTTPS (TLS) clients    | Optional |
| `--ssh`    | Backend for SSH clients            | Optional |
| `--rdp`    | Backend for RDP clients            | Optional |

> ✅ At least one backend must be configured.

## 📦 Example Use Case

Run Pux on the default RDP port (3389) and forward SSH, HTTP, or RDP clients to their correct backend automatically.

## 🧪 Testing

Try with `curl`, `ssh`, or an RDP client pointed at your mux port:

```bash
curl http://localhost:9999
ssh -p 9999 user@localhost
```

## 🛡️ Security

Pux doesn’t terminate TLS or SSH; it simply forwards traffic after protocol detection. That means end-to-end encryption is preserved.

## 📄 License

MIT

## 💡 Name Origin

**Pux** = **P**rotocol m**ux**er. Short, fast, and to the point — like the tool itself.