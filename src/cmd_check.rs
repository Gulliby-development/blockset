//! The check command: the list is valid, say how many networks it holds.

use crate::{report, Failure, Net, Options, Outcome};

pub fn run(_options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    Ok(Outcome { status: 0, out: format!("ok, {}\n", report::plural(items.len(), "network")), err: String::new() })
}
