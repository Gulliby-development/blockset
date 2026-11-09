//! The version, the limits, the usage text, the styles, the modes and the command table (in the order of the usage text).

use crate::Command;
use crate::{cmd_check, cmd_contains, cmd_count, cmd_gaps, cmd_merge, cmd_sort, cmd_split};

pub const VERSION: &str = "blockset 0.1.0";
pub const MAX_LINES: usize = 40;
pub const MAX_SPLIT: u64 = 256;
pub const USAGE: &str = "usage: blockset check\n       blockset sort [--style STYLE]\n       blockset merge [--style STYLE]\n       blockset count [--mode MODE]\n       blockset gaps --within NET [--style STYLE]\n       blockset contains --ip IP\n       blockset split --prefix N [--style STYLE]\n";
pub const STYLES: &[&str] = &["cidr", "range", "mask", "hex"];
pub const MODES: &[&str] = &["addresses", "hosts"];
pub const OPTIONS: &[&str] = &["--style", "--mode", "--within", "--ip", "--prefix"];

pub const COMMANDS: &[Command] = &[
    Command { name: "check", options: &[], required: "", handler: cmd_check::run },
    Command { name: "sort", options: &["--style"], required: "", handler: cmd_sort::run },
    Command { name: "merge", options: &["--style"], required: "", handler: cmd_merge::run },
    Command { name: "count", options: &["--mode"], required: "", handler: cmd_count::run },
    Command { name: "gaps", options: &["--within", "--style"], required: "--within", handler: cmd_gaps::run },
    Command { name: "contains", options: &["--ip"], required: "--ip", handler: cmd_contains::run },
    Command { name: "split", options: &["--prefix", "--style"], required: "--prefix", handler: cmd_split::run },
];
