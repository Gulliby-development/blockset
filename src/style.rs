//! The styles mask and hex.

mod masks;

use crate::{report, Failure};

/// A block in the style `mask` or `hex`.
pub fn render(first: u64, len: u32, style: &str) -> Result<String, Failure> {
    match style {
        "mask" => Ok(format!("{} {}", report::dotted(first), masks::of(len))),
        "hex" => Ok(format!("{first:08X}/{len}")),
        _ => Err(Failure::new(1, format!("unknown style {style}"))),
    }
}
