use std::cmp::Ordering;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut ans: Vec<usize> = vec![];
    let mut students: Vec<(usize, usize, usize)> = vec![(0, 0, 0); n];
    for i in 0..n {
        students[i].0 = i + 1;
        students[i].1 = a[i];
        students[i].2 = b[i];
    }
    students.sort_by(|a, b| {
        let second_cmp = a.1.cmp(&b.1);
        if second_cmp == Ordering::Equal {
            b.0.cmp(&a.0)
        } else {
            second_cmp
        }
    });
    students.reverse();
    for i in 0..n {
        if ans.len() == x {
            break;
        }
        ans.push(students[i].0);
    }
    students.sort_by(|a, b| {
        let second_cmp = a.2.cmp(&b.2);
        if second_cmp == Ordering::Equal {
            b.0.cmp(&a.0)
        } else {
            second_cmp
        }
    });
    students.reverse();
    for i in 0..n {
        if ans.contains(&students[i].0) {
            continue;
        }
        if ans.len() == x + y {
            break;
        }
        ans.push(students[i].0);
    }
    students.sort_by(|a, b| {
        let second_cmp = (a.1 + a.2).cmp(&(b.1 + b.2));
        if second_cmp == Ordering::Equal {
            b.0.cmp(&a.0)
        } else {
            second_cmp
        }
    });
    students.reverse();
    for i in 0..n {
        if ans.contains(&students[i].0) {
            continue;
        }
        if ans.len() == x + y + z {
            break;
        }
        ans.push(students[i].0);
    }
    ans.sort();

    println!("{}", ans.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
    );
}
