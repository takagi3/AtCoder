use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    for i in 0..=n {
        let mut kinds: Vec<bool> = vec![false; m + 1];
        kinds[0] = true;
        for j in 0..(n - i) {
            kinds[a[j]] = true;
        }
        if !kinds.iter().all(|&x| x) {
            println!("{}", i);
            break;
        }
    }
}
