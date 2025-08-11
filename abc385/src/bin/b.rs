use proconio::{input, marker::Chars};

fn dir(ch: char) -> Option<(isize, isize)> {
    match ch {
        'U' => Some((-1, 0)),
        'D' => Some((1, 0)),
        'L' => Some((0, -1)),
        'R' => Some((0, 1)),
        _ => None,
    }
}

fn step(s: &[Vec<char>], h: usize, w: usize, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    if (0..h as isize).contains(&nx) && (0..w as isize).contains(&ny) {
        let (ux, uy) = (nx as usize, ny as usize);
        if s[ux][uy] != '#' {
            return (ux, uy);
        }
    }
    (x, y)
}

fn collect(s: &mut [Vec<char>], x: usize, y: usize) -> usize {
    if s[x][y] == '@' {
        s[x][y] = '.';
        1
    } else {
        0
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: usize,
        mut y: usize,
        mut s: [Chars; h],
        t: Chars,
    }

    x -= 1;
    y -= 1;

    let mut c = collect(&mut s, x, y);

    for &ch in &t {
        if let Some(d) = dir(ch) {
            let (nx, ny) = step(&s, h, w, (x, y), d);
            x = nx;
            y = ny;
            c += collect(&mut s, x, y);
        }
    }

    println!("{} {} {}", x + 1, y + 1, c);
}
