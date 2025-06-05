use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [(i32, char); n],
    }

    let mut ans = 0;
    let mut last_l: Option<i32> = None;
    let mut last_r: Option<i32> = None;
    for &(pos, dir) in &a_s {
        match dir {
            'L' => {
                if let Some(prev) = last_l {
                    ans += (prev - pos).abs();
                }
                last_l = Some(pos);
            }
            'R' => {
                if let Some(prev) = last_r {
                    ans += (prev - pos).abs();
                }
                last_r = Some(pos);
            }
            _ => {}
        }
    }

    println!("{}", ans);
}
