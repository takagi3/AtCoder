use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }

    let mut ss: Vec<String> = vec![];
    let len: usize = s.len();
    for _ in 0..s.len() {
        let tmp = s[0];
        for j in 1..len {
            s[j - 1] = s[j];
        }
        s[len - 1] = tmp;
        ss.push(s.iter().collect::<String>());
    }

    let mut ans_min = &ss[0];
    let mut ans_max = &ss[0];
    for i in 1..ss.len() {
        if ans_min > &ss[i] {
            ans_min = &ss[i]
        }
        if ans_max < &ss[i] {
            ans_max = &ss[i]
        }
    }

    println!("{}\n{}", ans_min, ans_max);
}
