//! The reading of the list: lines, words and networks.

use crate::{nets, registry, Failure, Net};

/// The lines of the text: cut at LF, a final empty piece is no line, one CR at the end is dropped.
fn lines_of(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut pieces: Vec<&str> = text.split('\n').collect();
    if text.ends_with('\n') {
        pieces.pop();
    }
    pieces.into_iter().map(|line| line.strip_suffix('\r').unwrap_or(line)).collect()
}

/// The networks of the list, in input order.
pub fn read(text: &str) -> Result<Vec<Net>, Failure> {
    let lines = lines_of(text);
    if lines.len() > registry::MAX_LINES {
        return Err(Failure::new(1, format!("input is too long (max {} lines)", registry::MAX_LINES)));
    }
    let mut items = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let number = index + 1;
        let words: Vec<&str> = line.split([' ', '\t']).filter(|word| !word.is_empty()).collect();
        let Some(first) = words.first() else {
            continue;
        };
        if first.starts_with('#') {
            continue;
        }
        if words.len() > 1 {
            return Err(Failure::new(1, format!("line {number}: expected 1 network")));
        }
        let Some(net) = nets::parse(number, first) else {
            return Err(Failure::new(1, format!("line {number}: bad network {first}")));
        };
        if !nets::aligned(&net) {
            return Err(Failure::new(1, format!("line {number}: host bits set {first}")));
        }
        items.push(net);
    }
    if items.is_empty() {
        return Err(Failure::new(1, "no networks"));
    }
    Ok(items)
}
