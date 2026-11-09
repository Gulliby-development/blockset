//! The union of the networks as the fewest blocks.

use crate::Net;

/// Appends the fewest aligned blocks (first, len) that cover exactly lo..=hi, greedily.
pub fn decompose(lo: u64, hi: u64, out: &mut Vec<(u64, u32)>) {
    let mut lo = lo;
    while lo <= hi {
        let mut size: u64 = 1u64 << lo.trailing_zeros().min(32);
        while size > hi - lo + 1 {
            size >>= 1;
        }
        out.push((lo, 32 - size.trailing_zeros()));
        lo += size;
    }
}

/// The inclusive ranges of the networks, sorted by start, with overlapping or touching ones joined.
pub fn intervals(items: &[Net]) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = items.iter().map(|net| (net.first, net.last())).collect();
    ranges.sort_unstable();
    let mut joined: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        match joined.last_mut() {
            Some(prev) if start <= prev.1 + 1 => prev.1 = prev.1.max(end),
            _ => joined.push((start, end)),
        }
    }
    joined
}

/// The union of the list as the fewest blocks, ordered by first address.
pub fn blocks(items: &[Net]) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    for (lo, hi) in intervals(items) {
        decompose(lo, hi, &mut out);
    }
    out
}
