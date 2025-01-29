use proconio::input;

fn main() {
    input! {
        month: usize,
        day: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize,
    }

    d += 1;
    if d > day {
        m += 1;
        d = 1;
    }
    if m > month {
        y += 1;
        m = 1;
    }

    println!("{} {} {}", y, m, d);
}
