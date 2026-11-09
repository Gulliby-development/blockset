mod harness;

const ROWS: &[harness::Row] = &[
    (&["check"], "0.0.0.0\n", 0, "ok, 1 network\n", ""),
    (&["check"], "255.255.255.255\n", 0, "ok, 1 network\n", ""),
    (&["check"], "0.0.0.0/0\n", 0, "ok, 1 network\n", ""),
    (&["check"], "1.2.3.4/32\n", 0, "ok, 1 network\n", ""),
    (&["check"], "10.0.0.0/8\n", 0, "ok, 1 network\n", ""),
    (&["check"], "192.168.100.0/24\n", 0, "ok, 1 network\n", ""),
    (&["check"], "10.0.0.1/24\n", 1, "", "blockset: line 1: host bits set 10.0.0.1/24\n"),
    (&["check"], "10.0.0.0/7\n", 0, "ok, 1 network\n", ""),
    (&["check"], "10.0.0.2/31\n", 0, "ok, 1 network\n", ""),
    (&["check"], "10.0.0.1/31\n", 1, "", "blockset: line 1: host bits set 10.0.0.1/31\n"),
    (&["check"], "128.0.0.0/1\n", 0, "ok, 1 network\n", ""),
    (&["check"], "128.0.0.1/1\n", 1, "", "blockset: line 1: host bits set 128.0.0.1/1\n"),
    (&["check"], "256.0.0.0\n", 1, "", "blockset: line 1: bad network 256.0.0.0\n"),
    (&["check"], "1.2.3\n", 1, "", "blockset: line 1: bad network 1.2.3\n"),
    (&["check"], "1.2.3.4.5\n", 1, "", "blockset: line 1: bad network 1.2.3.4.5\n"),
    (&["check"], "01.2.3.4\n", 1, "", "blockset: line 1: bad network 01.2.3.4\n"),
    (&["check"], "1.2.3.04\n", 1, "", "blockset: line 1: bad network 1.2.3.04\n"),
    (&["check"], "1..3.4\n", 1, "", "blockset: line 1: bad network 1..3.4\n"),
    (&["check"], "a.b.c.d\n", 1, "", "blockset: line 1: bad network a.b.c.d\n"),
    (&["check"], "+1.2.3.4\n", 1, "", "blockset: line 1: bad network +1.2.3.4\n"),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
