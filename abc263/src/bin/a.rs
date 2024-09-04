use proconio::input;

fn main() {
    input! {
        n: [usize; 5],
    }

    let mut cnt: Vec<u32> = vec![0; 14];
    for i in 0..5 {
        cnt[n[i]] += 1
    }
    let mut ans: &str = "Yes";
    for i in 0..14 {
        if cnt[i] == 1 || cnt[i] == 4 {
            ans = "No"
        }
    }

    println!("{}", ans);
}
