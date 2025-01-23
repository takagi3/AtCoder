use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..d[i] {
            let tmp = format!("{}{}", i + 1, j + 1);
            let mut ok = true;
            for k in 0..tmp.len() - 1 {
                if tmp.chars().nth(k) != tmp.chars().nth(k + 1) {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
