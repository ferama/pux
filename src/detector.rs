use crate::protocol::Protocol;
use tls_parser::{
    SNIType, TlsExtension, TlsMessage, TlsMessageHandshake, parse_tls_extensions,
    parse_tls_plaintext,
};

fn detect_ssh(data: &[u8]) -> bool {
    // RFC 4253: SSH banner must start with "SSH-" and follow "SSH-protoversion-softwareversion"
    // e.g., "SSH-2.0-OpenSSH_8.2\r\n"

    if let Ok(text) = std::str::from_utf8(data) {
        if text.starts_with("SSH-") {
            // Grab the first line (banner)
            if let Some(banner_line) = text.lines().next() {
                // Match SSH protoversion: "SSH-2.0" or "SSH-1.99" (fallback version)
                return banner_line.starts_with("SSH-2.0") || banner_line.starts_with("SSH-1.99");
            }
        }
    }

    false
}

fn detect_http(data: &[u8]) -> bool {
    // Common HTTP methods (RFC 9110 + WebDAV + others)
    const METHODS: [&[u8]; 11] = [
        b"GET",
        b"POST",
        b"HEAD",
        b"PUT",
        b"DELETE",
        b"OPTIONS",
        b"TRACE",
        b"CONNECT",
        b"PATCH",
        b"PROPFIND",
        b"MKCOL",
    ];

    // Fast bail-out
    if data.len() < 4 {
        return false;
    }

    for method in METHODS {
        if data.starts_with(method) {
            // Ensure it's followed by a space or slash (e.g., "GET /")
            if let Some(&next) = data.get(method.len()) {
                if next == b' ' || next == b'/' {
                    return true;
                }
            }
        }
    }

    false
}

fn detect_https(data: &[u8]) -> bool {
    if data.len() <= 5 || data[0] != 0x16 || data[1] != 0x03 {
        return false;
    }

    let Ok((_rem, tls)) = parse_tls_plaintext(data) else {
        return false;
    };

    for msg in tls.msg {
        if let TlsMessage::Handshake(TlsMessageHandshake::ClientHello(ref hello)) = msg {
            let Some(ext_data) = hello.ext else {
                continue;
            };

            let Ok((_rem, extensions)) = parse_tls_extensions(ext_data) else {
                continue;
            };

            for ext in extensions {
                if let TlsExtension::SNI(sni_list) = ext {
                    if sni_list.iter().any(|sni| sni.0 == SNIType::HostName) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn detect_rdp(data: &[u8]) -> bool {
    if data.len() < 11 {
        return false;
    }

    // TPKT header check: version and reserved
    if data[0] != 0x03 || data[1] != 0x00 {
        return false;
    }

    // Extract TPKT length
    let tpkt_length = u16::from_be_bytes([data[2], data[3]]) as usize;
    if tpkt_length != data.len() {
        // Might be a fragmented packet, but we skip it here for robustness
        return false;
    }

    // Check for X.224 Data TPDU: length byte = 0x06, code = 0xE0
    if data[5] != 0xE0 {
        return false;
    }

    // RDP Negotiation Request starts at offset 11
    if data.len() >= 15
        && data[11] == 0x01   // Type: Negotiation Request
        && data[12] == 0x00   // Flags
        && data[13] == 0x08   // Length LSB
        && data[14] == 0x00
    {
        return true;
    }

    false
}

pub fn detect_protocol(data: &[u8]) -> Protocol {
    if detect_ssh(data) {
        return Protocol::Ssh;
    }

    if detect_http(data) {
        return Protocol::Http;
    }

    if detect_https(data) {
        return Protocol::Https;
    }

    if detect_rdp(data) {
        return Protocol::Rdp;
    }

    Protocol::Unknown
}
