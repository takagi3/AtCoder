use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }

    a.sort();
    println!(
        "{}",
        if n >= 2
            && a.windows(2)
                .any(|w| b.iter().all(|&x| x < w[0] || w[1] < x))
        {
            "Yes"
        } else {
            "No"
        }
    );
}
