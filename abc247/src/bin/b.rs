use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut name: Vec<String> = vec![];
    for i in 0..n {
        name.push(st[i].0.clone());
        if st[i].0 != st[i].1 {
            name.push(st[i].1.clone());
        }
    }
    let mut ok: bool = true;
    for i in 0..n {
        let mut cnt_s: usize = 0;
        let mut cnt_t: usize = 0;
        for j in 0..name.len() {
            if st[i].0 == name[j] {
                cnt_s += 1
            }
            if st[i].1 == name[j] {
                cnt_t += 1
            }
        }
        if cnt_s > 1 && cnt_t > 1 {
            ok = false;
            break;
        }
    }
    let ans: &str = if ok { "Yes" } else { "No" };

    println!("{}", ans);
}
