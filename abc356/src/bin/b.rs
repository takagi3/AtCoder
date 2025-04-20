use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; _n],
    }

    println!(
        "{}",
        if (0..m).all(|j| x.iter().map(|row| row[j]).sum::<usize>() >= a[j]) {
            "Yes"
        } else {
            "No"
        }
    );
}
