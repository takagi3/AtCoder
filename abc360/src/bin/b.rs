use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s.len() == 1 && s == t {
        println!("Yes");
        return;
    }
    let s: Vec<char> = s.chars().collect();
    let s_len = s.len();
    let t = t.as_str();
    for i in 1..s_len {
        for k in 1..=i {
            let tmp: String = (1..)
                .map(|j| i * j - k)
                .take_while(|&idx| idx < s_len)
                .map(|idx| s[idx])
                .collect();

            if tmp == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}