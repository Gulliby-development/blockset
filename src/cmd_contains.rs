//! The contains command: the networks of the list that hold an address.

use crate::{nets, report, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let ip = nets::address(&options.ip).ok_or_else(|| Failure::new(1, "bad value for --ip"))?;
    let mut out = String::new();
    for item in items {
        if item.first <= ip && ip <= item.last() {
            out.push_str(&format!("{} in {}\n", options.ip, report::render(item.first, item.len, "cidr")?));
        }
    }
    if out.is_empty() {
        return Ok(Outcome { status: 1, out: format!("{} is not covered\n", options.ip), err: String::new() });
    }
    Ok(Outcome { status: 0, out, err: String::new() })
}
