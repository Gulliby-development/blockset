use std::io::{Read, Write};

/// Standard input and output are bytes; every byte is one character (ISO 8859-1), as the rows of the tests read them.
fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&byte| char::from(byte)).collect()
}

fn bytes_of(text: &str) -> Vec<u8> {
    text.chars().map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?')).collect()
}

fn main() {
    let args: Vec<String> = std::env::args_os().skip(1).map(|arg| arg.to_string_lossy().into_owned()).collect();
    let mut input = String::new();
    if blockset::cli::reads_stdin(&args) {
        let mut raw = Vec::new();
        std::io::stdin().read_to_end(&mut raw).expect("cannot read standard input");
        input = latin1(&raw);
    }
    let outcome = blockset::cli::run(&args, &input);
    std::io::stdout().write_all(&bytes_of(&outcome.out)).expect("cannot write standard output");
    std::io::stderr().write_all(&bytes_of(&outcome.err)).expect("cannot write standard error");
    std::process::exit(outcome.status);
}
