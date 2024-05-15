use proconio::input;

const MAT2: [[u64; 4]; 4] = [
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 1, 0, 0],
    [1, 0, 0, 1]
];

const MAT3: [[u64; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0]
];

const MAT4: [[u64; 16]; 16] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1]
];

#[derive(Clone)]
struct Matrix {
    size: usize,
    p: [[u64; 16]; 16],
}

impl Matrix {
    fn new(size: usize) -> Self {
        Matrix {
            size,
            p: [[0; 16]; 16],
        }
    }
}

fn multiplication(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = Matrix::new(a.size);
    for i in 0..a.size {
        for k in 0..a.size {
            for j in 0..a.size {
                c.p[i][j] += a.p[i][k] * b.p[k][j];
                c.p[i][j] %= 1_000_000_007;
            }
        }
    }
    c
}

fn power(a: Matrix, n: u64) -> Matrix {
    let mut p = a.clone();
    let mut q = Matrix::new(a.size);
    let mut flag = false;
    for i in 0..60 {
        if (n & (1 << i)) != 0 {
            if !flag {
                q = p.clone();
                flag = true;
            } else {
                q = multiplication(&q, &p);
            }
        }
        p = multiplication(&p, &p);
    }
    q
}

fn main() {
    input! {
        k: u64,
        n: u64,
    }

    let size = 1 << k;
    let mut a = Matrix::new(size);
    for i in 0..size {
        for j in 0..size {
            a.p[i][j] = match k {
                2 => MAT2[i][j],
                3 => MAT3[i][j],
                4 => MAT4[i][j],
                _ => 0,
            };
        }
    }

    let b = power(a, n);
    println!("{}", b.p[size - 1][size - 1]);
}
