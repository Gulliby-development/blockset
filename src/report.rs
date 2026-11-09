//! The writing of a block in a style, and counts with their noun.

use crate::{style, Failure};

/// An address as four dotted numbers.
pub fn dotted(value: u64) -> String {
    format!("{}.{}.{}.{}", (value >> 24) & 255, (value >> 16) & 255, (value >> 8) & 255, value & 255)
}

/// A block in a style: cidr and range here, the others through `style::render`.
pub fn render(first: u64, len: u32, style_name: &str) -> Result<String, Failure> {
    match style_name {
        "cidr" => Ok(format!("{}/{}", dotted(first), len)),
        "range" => {
            let last = first + (1u64 << (32 - len)) - 1;
            Ok(format!("{}-{}", dotted(first), dotted(last)))
        }
        _ => style::render(first, len, style_name),
    }
}

/// `1 noun` or `N nouns`.
pub fn plural(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}
