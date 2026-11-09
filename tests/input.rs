mod harness;

const ROWS: &[harness::Row] = &[
    (&["check"], "", 1, "", "blockset: no networks\n"),
    (&["check"], "\n", 1, "", "blockset: no networks\n"),
    (&["check"], "# only\n\n   \n", 1, "", "blockset: no networks\n"),
    (&["check"], "10.0.0.0/8", 0, "ok, 1 network\n", ""),
    (&["check"], "10.0.0.0/8\n11.0.0.0/8", 0, "ok, 2 networks\n", ""),
    (&["check"], "10.0.0.0/8\r\n11.0.0.0/8\r\n", 0, "ok, 2 networks\n", ""),
    (&["check"], "10.0.0.0/8\n10.0.0.0/8 11.0.0.0/8\n", 1, "", "blockset: line 2: expected 1 network\n"),
    (&["check"], "\n# c\n10.0.0.0/8\n10.0.0.0 /8\n", 1, "", "blockset: line 4: expected 1 network\n"),
    (&["check"], "  10.0.0.0/8  \n\t11.0.0.0/8\t\n", 0, "ok, 2 networks\n", ""),
    (&["check"], "10.0.0.0/8\n   # note\n11.0.0.0/8\n#12.0.0.0/8\n", 0, "ok, 2 networks\n", ""),
    (&["check"], "10.0.0.0/8\nfrob\n11.0.0.0/8\nbad\n", 1, "", "blockset: line 2: bad network frob\n"),
    (&["check"], "10.0.0.0/8\nx\n1 2\n", 1, "", "blockset: line 2: bad network x\n"),
    (&["check"], "10.0.0.0/8\n10.0.0.0/8 #c\n", 1, "", "blockset: line 2: expected 1 network\n"),
    (&["check"], "10.0.0.0/8\n\n\n1.2.3\n", 1, "", "blockset: line 4: bad network 1.2.3\n"),
    (&["sort"], "10.0.0.0/8\n\t\n", 0, "10.0.0.0/8\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
