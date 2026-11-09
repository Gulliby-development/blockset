//! The reading of one address, one prefix length and one network word.

use crate::Net;

/// Digits only, 1 to `max_digits` of them, no leading zero unless the text is `0`.
fn number(text: &str, max_digits: usize) -> Option<u64> {
    if text.is_empty() || text.len() > max_digits || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if text.len() > 1 && text.starts_with('0') {
        return None;
    }
    text.parse().ok()
}

/// An address of four numbers as a `u64`.
pub fn address(text: &str) -> Option<u64> {
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let mut value = 0u64;
    for part in parts {
        let n = number(part, 3)?;
        if n > 255 {
            return None;
        }
        value = (value << 8) | n;
    }
    Some(value)
}

/// A prefix length from 0 to 32.
pub fn length(text: &str) -> Option<u32> {
    let n = number(text, 2)?;
    if n > 32 {
        return None;
    }
    u32::try_from(n).ok()
}

/// One network word (syntax only; host bits are not checked here).
pub fn parse(line: usize, word: &str) -> Option<Net> {
    let (addr, len) = match word.split_once('/') {
        Some((addr, len)) => (addr, length(len)?),
        None => (word, 32),
    };
    let first = address(addr)?;
    Some(Net { line, word: word.to_string(), first, len })
}

/// True when the network has no host bits set.
pub fn aligned(net: &Net) -> bool {
    net.first.is_multiple_of(net.size())
}
