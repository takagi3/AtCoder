use proconio::input;

fn main() {
    input! {
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    }

    let ans: char;
    loop {
        if v >= a {
            v -= a
        } else {
            ans = 'F';
            break;
        }

        if v >= b {
            v -= b
        } else {
            ans = 'M';
            break;
        }

        if v >= c {
            v -= c
        } else {
            ans = 'T';
            break;
        }
    }

    println!("{}", ans);
}
