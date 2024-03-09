use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        mut a: [u64; n],
    }

    a.sort();
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    let mut mid: usize;
    while left <= right {
        mid = (left + right) / 2;
        if a[mid] == x {
            println!("Yes");
            return;
        } else if a[mid] > x {
            if right == 0 { break; }
            right = mid - 1;
        } else if a[mid] < x {
            left = mid + 1;
        }
    }

    println!("No");
}
