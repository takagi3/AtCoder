use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut data = Vec::new();
    for _ in 0..n {
        input! {
            c: usize,
            a: [usize; c],
        }
        data.push((c, a));
    }
    input! {
        x: usize,
    }

    let mut min_c = usize::MAX;
    let mut ans = Vec::new();
    for (i, (c, a)) in data.into_iter().enumerate() {
        if a.contains(&x) {
            if c < min_c {
                min_c = c;
                ans.clear();
            }
            if c == min_c {
                ans.push(i + 1);
            }
        }
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!(
            "{}",
            ans.iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
