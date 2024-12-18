use proconio::input;

fn takcode(y: usize, x: usize, s: &[Vec<char>]) -> bool {
    for (i, j) in (0..3).flat_map(|i| (0..3).map(move |j| (i, j))) {
        if s[y + i][x + j] == '.' || s[y + 6 + i][x + 6 + j] == '.' {
            return false;
        }
    }
    let ranges = [
        (3..4, 0..4),
        (0..4, 3..4),
        (5..6, 5..9),
        (5..9, 5..6),
    ];
    for (i_range, j_range) in &ranges {
        for i in i_range.clone() {
            for j in j_range.clone() {
                if s[y + i][x + j] == '#' {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }

    let s: Vec<Vec<char>> = s.into_iter().map(|row| row.chars().collect()).collect();

    for i in 0..=n - 9 {
        for j in 0..=m - 9 {
            if takcode(i, j, &s) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
