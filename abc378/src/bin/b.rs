use proconio::input;

fn main() {
    input! {
        n: usize,
        qr: [(i64, i64); n],
        query: usize,
        td: [(usize, i64); query],
    }

    for (t, d) in td {
        let (q, r) = qr[t - 1];
        println!("{}", d + (r + q - d % q) % q);
    }
}
