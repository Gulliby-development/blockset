//! blockset: sort, merge, count, split and search lists of IPv4 networks.

pub mod cli;
pub mod cmd_check;
pub mod cmd_contains;
pub mod cmd_count;
pub mod cmd_gaps;
pub mod cmd_merge;
pub mod cmd_sort;
pub mod cmd_split;
pub mod input;
pub mod merge;
pub mod nets;
pub mod registry;
pub mod report;
pub mod style;

/// What one run of the program gives back: the exit status and the two output streams.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub status: i32,
    pub out: String,
    pub err: String,
}

/// An error that ends a command: `cli::run` turns it into `blockset: MESSAGE` on standard error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub status: i32,
    pub message: String,
}

impl Failure {
    pub fn new(status: i32, message: impl Into<String>) -> Failure {
        Failure { status, message: message.into() }
    }
}

/// One network of the list: its line number, the word as written, the first address and the prefix length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Net {
    pub line: usize,
    pub word: String,
    pub first: u64,
    pub len: u32,
}

impl Net {
    /// The number of addresses of the network.
    pub fn size(&self) -> u64 {
        1u64 << (32 - self.len)
    }

    /// The last address of the network.
    pub fn last(&self) -> u64 {
        self.first + self.size() - 1
    }
}

/// The options of a command line, with the defaults filled in: style cidr, mode addresses; `within` and `ip` are empty and `prefix` is `None` when absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub style: String,
    pub mode: String,
    pub within: String,
    pub ip: String,
    pub prefix: Option<u32>,
}

/// A command returns its `Outcome` or a `Failure`; `items` is the list that `input::read` returns, in input order.
pub type Handler = fn(&Options, &[Net]) -> Result<Outcome, Failure>;

/// A command: its name, the options it accepts, the option that must be given (or `""`) and its handler.
pub struct Command {
    pub name: &'static str,
    pub options: &'static [&'static str],
    pub required: &'static str,
    pub handler: Handler,
}
