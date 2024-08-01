use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut ans: Vec<String> = vec![String::new(); a * n];
    let tile: Vec<&str> = vec![".", "#"];
    let mut c: usize;
    for i in 0..a * n {
        if (i / a) % 2 == 0 {
            c = 0
        } else {
            c = 1
        }
        for _ in 0..n {
            for _ in 0..b {
                ans[i].push_str(tile[c])
            }
            if c == 0 {
                c = 1
            } else {
                c = 0
            }
        }
    }

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
