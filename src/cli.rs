//! The command line: help, version, the option rules, the reading of the list and the call of the command.

use crate::registry::{COMMANDS, MODES, OPTIONS, STYLES, USAGE, VERSION};
use crate::{input, nets, Command, Failure, Options, Outcome};

/// What the words of the command line mean.
enum Parsed {
    Help,
    Version,
    NoWord,
    Error(String),
    Valid(&'static Command, Options),
}

/// Checks one option value and stores it; false when the value is bad.
fn apply(option: &str, value: &str, options: &mut Options) -> bool {
    match option {
        "--style" if STYLES.contains(&value) => options.style = value.to_string(),
        "--mode" if MODES.contains(&value) => options.mode = value.to_string(),
        "--prefix" => match nets::length(value) {
            Some(len) => options.prefix = Some(len),
            None => return false,
        },
        "--within" => match nets::parse(0, value) {
            Some(net) if nets::aligned(&net) => options.within = value.to_string(),
            _ => return false,
        },
        "--ip" => {
            if nets::address(value).is_none() {
                return false;
            }
            options.ip = value.to_string();
        }
        _ => return false,
    }
    true
}

/// The options after the command word, left to right; the first broken rule is the error.
fn parse_options(command: &'static Command, args: &[String]) -> Result<Options, String> {
    let mut options = Options { style: "cidr".to_string(), mode: "addresses".to_string(), within: String::new(), ip: String::new(), prefix: None };
    let mut seen: Vec<&str> = Vec::new();
    let mut index = 1;
    while index < args.len() {
        let word = args[index].as_str();
        if !word.starts_with('-') || word == "-" {
            return Err(format!("unexpected argument {word}"));
        }
        if !OPTIONS.contains(&word) {
            return Err(format!("unknown option {word}"));
        }
        if !command.options.contains(&word) {
            return Err(format!("option {word} is not valid for {}", command.name));
        }
        if seen.contains(&word) {
            return Err(format!("given twice: {word}"));
        }
        let Some(value) = args.get(index + 1) else {
            return Err(format!("missing value for {word}"));
        };
        if !apply(word, value, &mut options) {
            return Err(format!("bad value for {word}"));
        }
        seen.push(word);
        index += 2;
    }
    if !command.required.is_empty() && !seen.contains(&command.required) {
        return Err(format!("missing {}", command.required));
    }
    Ok(options)
}

fn parse(args: &[String]) -> Parsed {
    let Some(first) = args.first() else {
        return Parsed::NoWord;
    };
    match first.as_str() {
        "help" | "-h" | "--help" => return Parsed::Help,
        "--version" => return Parsed::Version,
        _ => {}
    }
    let Some(command) = COMMANDS.iter().find(|command| command.name == first.as_str()) else {
        return Parsed::Error(format!("unknown command {first}"));
    };
    match parse_options(command, args) {
        Ok(options) => Parsed::Valid(command, options),
        Err(message) => Parsed::Error(message),
    }
}

/// True when the command line is a valid use of a command, so the list must be read.
pub fn reads_stdin(args: &[String]) -> bool {
    matches!(parse(args), Parsed::Valid(..))
}

fn failed(status: i32, message: &str) -> Outcome {
    Outcome { status, out: String::new(), err: format!("blockset: {message}\n") }
}

fn execute(command: &Command, options: &Options, stdin: &str) -> Result<Outcome, Failure> {
    let items = input::read(stdin)?;
    (command.handler)(options, &items)
}

pub fn run(args: &[String], stdin: &str) -> Outcome {
    match parse(args) {
        Parsed::Help => Outcome { status: 0, out: USAGE.to_string(), err: String::new() },
        Parsed::Version => Outcome { status: 0, out: format!("{VERSION}\n"), err: String::new() },
        Parsed::NoWord => Outcome { status: 2, out: String::new(), err: USAGE.to_string() },
        Parsed::Error(message) => failed(2, &message),
        Parsed::Valid(command, options) => match execute(command, &options, stdin) {
            Ok(outcome) => outcome,
            Err(failure) => failed(failure.status, &failure.message),
        },
    }
}
