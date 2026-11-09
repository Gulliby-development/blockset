//! The merge command: the fewest networks that cover the same addresses.

use crate::{merge, report, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let mut out = String::new();
    for (first, len) in merge::blocks(items) {
        out.push_str(&report::render(first, len, &options.style)?);
        out.push('\n');
    }
    Ok(Outcome { status: 0, out, err: String::new() })
}
