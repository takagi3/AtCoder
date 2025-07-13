use proconio::input;

fn main() {
    input! {
        a: [usize; 4],
    }

    let ball = a.iter().fold([0u32; 5], |mut cnt, &x| {
        cnt[x] += 1;
        cnt
    });

    println!(
        "{}",
        ball.iter()
            .map(|&c| match c {
                4 => 2,
                2..=3 => 1,
                _ => 0,
            })
            .sum::<u32>()
    );
}
