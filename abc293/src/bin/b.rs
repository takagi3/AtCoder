use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut call = vec![true; n];
    for i in 0..n {
        if call[i] {
            call[a[i]] = false;
        }
    }
    let ans: Vec<usize> = call.iter()
        .enumerate()
        .filter_map(|(i, &called)| if called { Some(i + 1) } else { None })
        .collect();

    println!("{}", ans.len());
    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
