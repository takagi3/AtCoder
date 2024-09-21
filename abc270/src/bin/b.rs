use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut ans: i32 = 0;
    if 0 < x {
        if x < y || y < 0 {
            ans += x
        } else {
            if z < 0 {
                ans += -2 * z + x
            } else if z < y {
                ans += x
            } else {
                ans = -1
            }
        }
    } else {
        if x > y || y > 0 {
            ans -= x
        } else {
            if z > 0 {
                ans += 2 * z - x
            } else if z > y {
                ans -= x
            } else {
                ans = -1
            }
        }
    }

    println!("{}", ans);
}
