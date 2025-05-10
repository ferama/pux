// src/protocol.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Ssh,
    Http,
    Https,
    Rdp,
    Unknown,
}
