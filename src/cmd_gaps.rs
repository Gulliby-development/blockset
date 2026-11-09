//! The gaps command: the addresses of a network that the list does not cover.

use crate::{merge, nets, report, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let within = nets::parse(0, &options.within).ok_or_else(|| Failure::new(1, "bad value for --within"))?;
    let (lo, hi) = (within.first, within.last());
    let mut blocks: Vec<(u64, u32)> = Vec::new();
    let mut next = lo;
    for (start, end) in merge::intervals(items) {
        if end < lo || start > hi {
            continue;
        }
        let start = start.max(lo);
        let end = end.min(hi);
        if start > next {
            merge::decompose(next, start - 1, &mut blocks);
        }
        next = end + 1;
    }
    if next <= hi {
        merge::decompose(next, hi, &mut blocks);
    }
    let mut out = String::new();
    for (first, len) in blocks {
        out.push_str(&report::render(first, len, &options.style)?);
        out.push('\n');
    }
    if out.is_empty() {
        out.push_str("none\n");
    }
    Ok(Outcome { status: 0, out, err: String::new() })
}
