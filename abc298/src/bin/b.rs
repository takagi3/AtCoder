use proconio::input;

fn rotate(matrix: &mut Vec<Vec<u32>>) {
    let n: usize = matrix.len();
    for i in 0..n {
        for j in i + 1..n {
            matrix[i][j] ^= matrix[j][i];
            matrix[j][i] ^= matrix[i][j];
            matrix[i][j] ^= matrix[j][i];
        }
    }
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn can_match(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>) -> bool {
    a.iter()
        .zip(b.iter())
        .all(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).all(|(&a, &b)| a == 0 || b == 1))
}

fn main() {
    input! {
        n: usize,
        mut a: [[u32; n]; n],
        b: [[u32; n]; n],
    }

    let mut ans: &str = "No";
    for _ in 0..4 {
        if can_match(&a, &b) {
            ans = "Yes";
            break;
        }
        rotate(&mut a);
    }

    println!("{}", ans);
}
