use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ans: &str = "Yes";
    let mut cnt: HashMap<usize, u32> = HashMap::new();
    for i in 0..n {
        *cnt.entry(a[i]).or_default() += 1
    }
    for i in 0..m {
        if cnt.get(&b[i]) == None || cnt.get(&b[i]) == Some(&0) {
            ans = "No";
            break;
        } else {
            *cnt.entry(b[i]).or_default() -= 1
        }
    }

    println!("{}", ans);
}
