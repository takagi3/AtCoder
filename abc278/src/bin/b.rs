use proconio::input;

fn main() {
    input! {
        h: u32,
        mut m: u32,
    }

    let mut ans: (u32, u32) = (0, 0);

    'nest: for i in h..h + 24 {
        for j in m..m + 60 {
            if j == 60 {
                m = 0;
                break;
            }
            if i % 24 % 10 <= 5 && i % 24 / 10 < 2 || i % 24 / 10 == 2 && j / 10 <= 3 {
                ans.0 = i % 24;
                ans.1 = j;
                break 'nest;
            }
        }
    }

    println!("{} {}", ans.0, ans.1);
}
