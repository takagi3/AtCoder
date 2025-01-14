use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(i64, i64); n],
    }

    let mut x: Vec<i64> = wx.iter().map(|&(_, xi)| xi).collect();
    let w: Vec<i64> = wx.iter().map(|&(wi, _)| wi).collect();
    let mut ans = 0;
    for _ in 0..24 {
        let cnt: i64 = x.iter()
            .zip(&w)
            .filter(|(&xi, _)| (9..18).contains(&xi))
            .map(|(_, &wi)| wi)
            .sum();
        ans = ans.max(cnt);
        x.iter_mut().for_each(|xi| {
            *xi = (*xi + 1) % 24;
        });
    }

    println!("{}", ans);
}