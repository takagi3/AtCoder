use proconio::input;

const MOD: u64 = 1_000_000_007;

#[derive(Clone)]
struct Matrix {
    p: [[u64; 3]; 3],
}

impl Matrix {
    fn new() -> Self {
        Matrix { p: [[0, 0, 0], [0, 0, 0], [0, 0, 0]] }
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new();
        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    result.p[i][j] += self.p[i][k] * other.p[k][j];
                    result.p[i][j] %= MOD;
                }
            }
        }
        result
    }
}

fn matrix_power(mut base: Matrix, exponent: u64) -> Matrix {
    let mut result = Matrix::new();
    let mut initialized = false;

    for i in 0..60 {
        if (exponent & (1 << i)) != 0 {
            if !initialized {
                result = base.clone();
                initialized = true;
            } else {
                result = result.multiply(&base);
            }
        }
        base = base.multiply(&base);
    }
    result
}

fn main() {
    input! {
        n: u64,
    }

    let mut a = Matrix::new();
    a.p[0][0] = 1;
    a.p[0][1] = 1;
    a.p[0][2] = 1;
    a.p[1][0] = 1;
    a.p[2][1] = 1;

    let result = matrix_power(a, n - 1);
    println!("{}", (2 * result.p[2][0] + result.p[2][1] + result.p[2][2]) % MOD);
}
