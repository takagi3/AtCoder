use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        si: usize,
        sj: usize,
        grid: [Chars; h],
        moves: Chars,
    }

    let (y, x) = moves.into_iter().fold((si - 1, sj - 1), |(y, x), m| {
        let (ny, nx) = match m {
            'L' => (y, x.saturating_sub(1)),
            'R' => (y, (x + 1).min(w - 1)),
            'U' => (y.saturating_sub(1), x),
            'D' => ((y + 1).min(h - 1), x),
            _ => (y, x),
        };
        if grid[ny][nx] == '.' {
            (ny, nx)
        } else {
            (y, x)
        }
    });

    println!("{} {}", y + 1, x + 1);
}
