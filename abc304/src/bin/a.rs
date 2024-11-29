use proconio::input;

fn main() {
    input! {
        n: usize,
        mut player: [(String, u32); n],
    }

    let head = player
        .iter()
        .enumerate()
        .min_by_key(|&(_, &(_, value))| value)
        .map(|(index, _)| index)
        .unwrap();
    player.rotate_left(head);

    for (name, _) in player {
        println!("{}", name);
    }
}
