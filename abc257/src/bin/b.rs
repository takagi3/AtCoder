use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q],
    }

    let mut grid: Vec<bool> = vec![false; n + 1];
    for i in 0..k {
        grid[a[i]] = true
    }
    for i in 0..q {
        if a[l[i] - 1] == n || grid[a[l[i] - 1] + 1] {
            continue;
        }
        grid[a[l[i] - 1]] = false;
        grid[a[l[i] - 1] + 1] = true;
        a[l[i] - 1] += 1;
    }

    println!("{}", a.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
    );
}
