//! The split command: every network cut into networks of the given prefix length.

use crate::{registry, report, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let Some(n) = options.prefix else {
        return Err(Failure::new(1, "missing --prefix"));
    };
    for net in items {
        if net.len > n {
            return Err(Failure::new(1, format!("line {}: cannot split {} into /{}", net.line, net.word, n)));
        }
    }
    let total: u64 = items.iter().map(|net| 1u64 << (n - net.len)).sum();
    if total > registry::MAX_SPLIT {
        return Err(Failure::new(1, "too many subnets (max 256)"));
    }
    let step = 1u64 << (32 - n);
    let mut out = String::new();
    for net in items {
        for i in 0..(1u64 << (n - net.len)) {
            out.push_str(&report::render(net.first + i * step, n, &options.style)?);
            out.push('\n');
        }
    }
    Ok(Outcome { status: 0, out, err: String::new() })
}
