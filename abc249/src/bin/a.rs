use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        x: u32,
    }

    let mut dist_t: u32 = x / (a + c) * (a * b);
    dist_t = if x % (a + c) >= a { dist_t + (a * b) } else { dist_t + x % (a + c) * b };
    let mut dist_a: u32 = x / (d + f) * (d * e);
    dist_a = if x % (d + f) >= d { dist_a + (d * e) } else { dist_a + x % (d + f) * e };
    let mut ans: &str = "Draw";
    if dist_t > dist_a {
        ans = "Takahashi"
    } else if dist_t < dist_a {
        ans = "Aoki"
    }

    println!("{}", ans);
}
