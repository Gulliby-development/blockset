<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# blockset

[![CI](https://github.com/Gulliby-development/blockset/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/blockset/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

blockset is a small command-line tool written in Rust with the standard library only. It reads a list of IPv4 networks on standard input, one network per line, and can validate the list, sort it, merge it, count the addresses it covers, list the addresses of a network that the list does not cover, name the networks that hold an address, or cut every network into smaller ones.

It is for people who keep lists of networks (firewall rules, routing tables, allow lists) and use the tool in a terminal, in a script or in CI. It needs no network and no clock.

## Usage

Build once and run the binary from the repository directory:

```text
cargo build
target/debug/blockset COMMAND
```

or run it through cargo with `cargo run -q -- COMMAND`.

The seven commands, in the order of the usage text:

- `blockset check`
- `blockset sort [--style STYLE]`
- `blockset merge [--style STYLE]`
- `blockset count [--mode MODE]`
- `blockset gaps --within NET [--style STYLE]`
- `blockset contains --ip IP`
- `blockset split --prefix N [--style STYLE]`

`help`, `-h` and `--help` as the first word print this usage on standard output and exit 0, whatever follows. `--version` prints `blockset 0.1.0` and exits 0. With no word at all the usage goes to standard error and the exit status is 2.

Option rules:

- Every option takes the next word as its value. There is no `--option=value` form.
- `--style` is one of `cidr`, `range`, `mask`, `hex`; the default is `cidr`.
- `--mode` is one of `addresses`, `hosts`; the default is `addresses`.
- `--prefix` is a whole number from 0 to 32 with no leading zero.
- `--within` is one network, written as a line of the list is.
- `--ip` is one address (four numbers, no `/`).
- `--within`, `--ip` and `--prefix` must be given for `gaps`, `contains` and `split`.

## Networks

The list is read from standard input:

- Lines end at a line feed; a final line without one still counts, and one carriage return at the end of a line is dropped.
- At most 40 lines are allowed, skipped lines included.
- A line that holds only spaces and tabs, or whose first word starts with `#`, is skipped.
- Every other line holds exactly one word; spaces and tabs around it are ignored.
- A list without any network is an error.

A network is an address, or an address followed by `/` and a prefix length. An address is four numbers separated by dots, each from 0 to 255 with no leading zero and no sign. A prefix length is a number from 0 to 32 with no leading zero. An address without `/` is a network of one address (length 32). A network must not have host bits set: `10.0.0.1/24` is rejected, `10.0.0.0/24` is fine.

## Styles

Every command that writes networks accepts `--style`:

- `cidr`: address, `/` and length, for example `10.0.0.0/8`.
- `range`: first and last address, for example `10.0.0.0-10.255.255.255`.
- `mask`: first address, a space and the netmask, for example `10.0.0.0 255.0.0.0`.
- `hex`: the first address as eight upper-case hexadecimal digits, `/` and the length, for example `0A000000/8`.

## Output

A command that works writes its result to standard output and nothing to standard error.

- `check` prints `ok, N networks` (`ok, 1 network` for one).
- `sort` prints every network, ordered by first address and then by prefix length (shorter first), one per line in the style.
- `merge` prints the fewest networks that cover exactly the same addresses, ordered by first address. Networks that overlap or touch are joined first; two neighbours that do not form one network stay two.
- `count` prints the number of covered addresses, each counted once. With `--mode hosts` each merged network counts its addresses minus 2 when its length is 30 or shorter, and all its addresses for length 31 or 32.
- `gaps` prints the addresses of the `--within` network that the list does not cover, as the fewest networks, or `none` when nothing is left. Networks outside it are ignored.
- `contains` prints `IP in NET` (NET in `cidr`) for each network that holds the address, in input order. When none does, it prints `IP is not covered` on standard output and exits 1.
- `split` cuts every network, in input order, into its networks of length N in ascending order. A network longer than N is the error `line L: cannot split WORD into /N`; more than 256 output lines is the error `too many subnets (max 256)`.

## Examples

In these examples `blockset` stands for a shell function that runs `target/debug/blockset "$@"`. They use a POSIX shell in the repository directory after `cargo build`; `2>&1` shows the error line.

```text
$ printf '10.0.0.0/25\n10.0.0.128/25\n10.0.1.0/24\n' | blockset merge
10.0.0.0/23
$ printf '10.0.0.0/24\n192.168.1.0/30\n' | blockset sort --style range; printf '172.16.0.0/12\n' | blockset sort --style mask
10.0.0.0-10.0.0.255
192.168.1.0-192.168.1.3
172.16.0.0 255.240.0.0
$ printf '10.0.0.0/24\n10.0.0.64/26\n' | blockset gaps --within 10.0.0.0/23; printf '10.0.0.0/30\n' | blockset split --prefix 31 --style hex
10.0.1.0/24
0A000000/31
0A000002/31
$ printf '10.0.0.0/8\n10.0.0.1/24\n' | blockset check 2>&1; printf '10.0.0.0/8\n' | blockset contains 2>&1
blockset: line 2: host bits set 10.0.0.1/24
blockset: missing --ip
```

## Errors

A failing command writes nothing to standard output and one line `blockset: MESSAGE` to standard error.

Exit statuses:

- 0: success, `help` and `--version`.
- 1: an error in the list or in a command, and `contains` when no network holds the address.
- 2: an error in the command line, or no argument at all. Standard input is not read.

Command-line errors (checked left to right, the first one counts):

- `blockset: unknown command WORD`
- `blockset: unexpected argument WORD`
- `blockset: unknown option WORD`
- `blockset: option OPTION is not valid for COMMAND`
- `blockset: given twice: OPTION`
- `blockset: missing value for OPTION`
- `blockset: bad value for OPTION`
- `blockset: missing --within`, `blockset: missing --ip`, `blockset: missing --prefix`

List errors (exit 1, the first problem wins, checked before the command runs):

- `blockset: input is too long (max 40 lines)`
- `blockset: line N: expected 1 network`
- `blockset: line N: bad network WORD`
- `blockset: line N: host bits set WORD`
- `blockset: no networks`

Command errors of `split` (exit 1):

- `blockset: line L: cannot split WORD into /N`
- `blockset: too many subnets (max 256)`

## Development

Run these before committing:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

blockset is released under the MIT license; see the `LICENSE` file.

---

Built with [BAKED](https://github.com/Gulliby-development/baked), an open-source coding agent (bench round R103).
