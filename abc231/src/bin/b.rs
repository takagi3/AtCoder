use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    for i in 0..n {
        if map.contains_key(&s[i]) {
            *map.get_mut(&s[i]).unwrap() += 1;
        } else {
            map.insert(s[i].clone(), 1);
        }
    }
    let mut max: usize = 0;
    let mut ans: &str = "";
    for (k, v) in map.iter() {
        if max < *v {
            ans = k;
            max = *v;
        }
    }

    println!("{}", ans);
}
