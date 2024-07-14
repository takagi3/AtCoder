use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ans: &str = "Yes";
    let mut cnt: HashMap<usize, u32> = HashMap::new();
    for i in 0..n {
        if cnt.get(&a[i]) == None {
            cnt.insert(a[i], 1);
        } else {
            cnt[&a[i]] += 1
        }
    }
    for i in 0..m {
        if cnt.get(&b[i]) == None || cnt[&b[i]] == 0 {
            ans = "No";
            break;
        } else {
            cnt[&b[i]] -= 1
        }
    }

    println!("{}", ans);
}
