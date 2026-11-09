//! The sort command: the networks ordered by first address, then by prefix length.

use crate::{report, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let mut sorted = items.to_vec();
    sorted.sort_by_key(|net| (net.first, net.len));
    let mut out = String::new();
    for net in &sorted {
        out.push_str(&report::render(net.first, net.len, &options.style)?);
        out.push('\n');
    }
    Ok(Outcome { status: 0, out, err: String::new() })
}
