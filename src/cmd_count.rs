//! The count command: the number of addresses (or hosts) that the list covers.

use crate::{merge, Failure, Net, Options, Outcome};

pub fn run(options: &Options, items: &[Net]) -> Result<Outcome, Failure> {
    let hosts = options.mode == "hosts";
    let mut total: u64 = 0;
    for (_, len) in merge::blocks(items) {
        let size = 1u64 << (32 - len);
        total += if hosts && len <= 30 { size - 2 } else { size };
    }
    Ok(Outcome { status: 0, out: format!("{}\n", total), err: String::new() })
}
