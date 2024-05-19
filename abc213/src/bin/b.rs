use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut player: Vec<(u32, usize)> = vec![(0, 0); n];
    for i in 0..n {
        player[i].0 = a[i];
        player[i].1 = i + 1;
    }
    player.sort();

    println!("{}", player[n - 2].1);
}
