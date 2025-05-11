use proconio::input;

fn main() {
    input! {
        (xa, ya): (i64, i64),
        (xb, yb): (i64, i64),
        (xc, yc): (i64, i64),
    }

    let mut d = [
        (xa - xb).pow(2) + (ya - yb).pow(2),
        (xa - xc).pow(2) + (ya - yc).pow(2),
        (xb - xc).pow(2) + (yb - yc).pow(2),
    ];
    d.sort_unstable();

    println!("{}", if d[0] + d[1] == d[2] { "Yes" } else { "No" });
}
