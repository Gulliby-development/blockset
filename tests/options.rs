mod harness;

const ROWS: &[harness::Row] = &[
    (&["sort", "extra"], "", 2, "", "blockset: unexpected argument extra\n"),
    (&["sort", "-"], "", 2, "", "blockset: unexpected argument -\n"),
    (&["sort", ""], "", 2, "", "blockset: unexpected argument \n"),
    (&["sort", "--bogus"], "", 2, "", "blockset: unknown option --bogus\n"),
    (&["check", "--style", "cidr"], "", 2, "", "blockset: option --style is not valid for check\n"),
    (&["sort", "--mode", "hosts"], "", 2, "", "blockset: option --mode is not valid for sort\n"),
    (&["sort", "--style", "cidr", "--style", "cidr"], "", 2, "", "blockset: given twice: --style\n"),
    (&["sort", "--style"], "", 2, "", "blockset: missing value for --style\n"),
    (&["sort", "-x", "--style"], "", 2, "", "blockset: unknown option -x\n"),
    (&["check", "--style"], "", 2, "", "blockset: option --style is not valid for check\n"),
    (&["sort", "--style", "x", "--bogus"], "", 2, "", "blockset: bad value for --style\n"),
    (&["count", "--style", "hex"], "", 2, "", "blockset: option --style is not valid for count\n"),
    (&["merge", "--prefix", "8"], "", 2, "", "blockset: option --prefix is not valid for merge\n"),
    (&["gaps"], "", 2, "", "blockset: missing --within\n"),
    (&["split"], "", 2, "", "blockset: missing --prefix\n"),
    (&["contains"], "", 2, "", "blockset: missing --ip\n"),
    (&["gaps", "--style", "mask"], "", 2, "", "blockset: missing --within\n"),
    (&["split", "--style", "hex", "--style", "mask"], "", 2, "", "blockset: given twice: --style\n"),
    (&["merge", "--style", "range"], "10.0.0.0/25\n10.0.0.128/25\n", 0, "10.0.0.0-10.0.0.255\n", ""),
    (&["count", "--mode", "hosts"], "10.0.0.0/24\n", 0, "254\n", ""),
    (&["sort", "--style", "hex"], "10.0.0.0/8\n", 0, "0A000000/8\n", ""),
    (&["split", "--style", "range", "--prefix", "25"], "10.0.0.0/24\n", 0, "10.0.0.0-10.0.0.127\n10.0.0.128-10.0.0.255\n", ""),
    (&["gaps", "--style", "cidr", "--within", "10.0.0.0/23"], "10.0.0.0/24\n", 0, "10.0.1.0/24\n", ""),
    (&["contains", "--ip", "9.9.9.9"], "9.0.0.0/8\n", 0, "9.9.9.9 in 9.0.0.0/8\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
