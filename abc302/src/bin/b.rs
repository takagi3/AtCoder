use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let target = "snuke".chars().collect::<Vec<_>>();
    let mut ans = Vec::new();
    'nest: for i in 0..h {
        for j in 0..w {
            for &(dx, dy) in &directions {
                let mut ok = true;
                let mut result = Vec::new();
                for step in 0..5 {
                    let nx = j as isize + dx * step;
                    let ny = i as isize + dy * step;
                    if nx < 0
                        || ny < 0
                        || nx >= w as isize
                        || ny >= h as isize
                        || s[ny as usize][nx as usize] != target[step as usize]
                    {
                        ok = false;
                        break;
                    }
                    result.push((ny + 1, nx + 1));
                }
                if ok {
                    ans = result;
                    break 'nest;
                }
            }
        }
    }

    for (y, x) in ans {
        println!("{} {}", y, x);
    }
}
