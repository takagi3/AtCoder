use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut ans: &str = "No";
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    let mut mid: usize;
    while left <= right {
        mid = (left + right) / 2;
        if a[mid] == x {
            ans = "Yes";
            break;
        } else if a[mid] < x {
            left = mid + 1;
        } else if a[mid] > x {
            if mid == 0 { break; }
            right = mid - 1;
        }
    }

    println!("{}", ans);
}
