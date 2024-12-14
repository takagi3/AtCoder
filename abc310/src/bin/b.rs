use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        _m: usize,
    }

    let mut ans: &str = "No";
    let mut price: Vec<usize> = vec![0; n];
    let mut func: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        input! {
            p: usize,
            fc: [usize],
        }
        price[i] = p;
        func[i] = fc;
    }
    'nest: for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if price[i] >= price[j] && func[i].len() <= func[j].len() {
                let set_i: HashSet<_> = func[i].iter().collect();
                let set_j: HashSet<_> = func[j].iter().collect();

                if set_i.is_subset(&set_j) && (price[i] > price[j] || func[i].len() < func[j].len())
                {
                    ans = "Yes";
                    break 'nest;
                }
            }
        }
    }

    println!("{}", ans);
}
