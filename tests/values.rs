mod harness;

const ROWS: &[harness::Row] = &[
    (&["sort", "--style", "Cidr"], "", 2, "", "blockset: bad value for --style\n"),
    (&["sort", "--style", ""], "", 2, "", "blockset: bad value for --style\n"),
    (&["count", "--mode", "all"], "", 2, "", "blockset: bad value for --mode\n"),
    (&["split", "--prefix", "33"], "", 2, "", "blockset: bad value for --prefix\n"),
    (&["split", "--prefix", "05"], "", 2, "", "blockset: bad value for --prefix\n"),
    (&["split", "--prefix", "-1"], "", 2, "", "blockset: bad value for --prefix\n"),
    (&["split", "--prefix", "2.5"], "", 2, "", "blockset: bad value for --prefix\n"),
    (&["split", "--prefix", ""], "", 2, "", "blockset: bad value for --prefix\n"),
    (&["gaps", "--within", "10.0.0.1/24"], "", 2, "", "blockset: bad value for --within\n"),
    (&["gaps", "--within", "300.0.0.0/8"], "", 2, "", "blockset: bad value for --within\n"),
    (&["gaps", "--within", "10.0.0.0/"], "", 2, "", "blockset: bad value for --within\n"),
    (&["gaps", "--within", "10.0.0.0/33"], "", 2, "", "blockset: bad value for --within\n"),
    (&["contains", "--ip", "1.2.3"], "", 2, "", "blockset: bad value for --ip\n"),
    (&["contains", "--ip", "1.2.3.4/5"], "", 2, "", "blockset: bad value for --ip\n"),
    (&["contains", "--ip", "01.2.3.4"], "", 2, "", "blockset: bad value for --ip\n"),
    (&["count", "--mode", "addresses"], "10.0.0.0/30\n", 0, "4\n", ""),
    (&["count", "--mode", "hosts"], "10.0.0.0/31\n10.0.0.2/32\n", 0, "3\n", ""),
    (&["split", "--prefix", "31", "--style", "range"], "10.0.0.0/30\n", 0, "10.0.0.0-10.0.0.1\n10.0.0.2-10.0.0.3\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
