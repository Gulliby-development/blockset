mod harness;

const ROWS: &[harness::Row] = &[
    (&["help"], "", 0, "usage: blockset check\n       blockset sort [--style STYLE]\n       blockset merge [--style STYLE]\n       blockset count [--mode MODE]\n       blockset gaps --within NET [--style STYLE]\n       blockset contains --ip IP\n       blockset split --prefix N [--style STYLE]\n", ""),
    (&["-h"], "", 0, "usage: blockset check\n       blockset sort [--style STYLE]\n       blockset merge [--style STYLE]\n       blockset count [--mode MODE]\n       blockset gaps --within NET [--style STYLE]\n       blockset contains --ip IP\n       blockset split --prefix N [--style STYLE]\n", ""),
    (&["--help", "--bogus"], "", 0, "usage: blockset check\n       blockset sort [--style STYLE]\n       blockset merge [--style STYLE]\n       blockset count [--mode MODE]\n       blockset gaps --within NET [--style STYLE]\n       blockset contains --ip IP\n       blockset split --prefix N [--style STYLE]\n", ""),
    (&[], "", 2, "", "usage: blockset check\n       blockset sort [--style STYLE]\n       blockset merge [--style STYLE]\n       blockset count [--mode MODE]\n       blockset gaps --within NET [--style STYLE]\n       blockset contains --ip IP\n       blockset split --prefix N [--style STYLE]\n"),
    (&["--version"], "", 0, "blockset 0.1.0\n", ""),
    (&["--version", "x"], "", 0, "blockset 0.1.0\n", ""),
    (&["frob"], "", 2, "", "blockset: unknown command frob\n"),
    (&[""], "", 2, "", "blockset: unknown command \n"),
    (&["Check"], "", 2, "", "blockset: unknown command Check\n"),
    (&["-H"], "", 2, "", "blockset: unknown command -H\n"),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
