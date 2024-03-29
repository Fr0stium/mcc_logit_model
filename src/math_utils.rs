/// Evaluates the hazard rate of the probability distribution.
pub fn hazard(x: f64) -> f64 {
    cdf(x)
}

/// Evaluates the derivative of the hazard rate of the probability distribution.
pub fn hazard_prime(x: f64) -> f64 {
    pdf(x)
}

/// Evaluates the density function of the probability distribution.
fn pdf(x: f64) -> f64 {
    (-x).exp() / ((-x).exp() + 1.0).powi(2)
}

/// Evaluates the distribution function of the probability distribution.
fn cdf(x: f64) -> f64 {
    (1.0 + (-x).exp()).recip()
}

/// For a matrix equation Ax=b, this function takes the augmented matrix (A|b) and returns the solution x.
pub fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Vec<f64> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);
    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }
    for i in (1..size).rev() {
        eliminate(matrix, i);
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == 0f64 {
            println!("Infinitely many solutions");
        }
    }
    let mut result: Vec<f64> = vec![0f64; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

fn echelon(matrix: &mut [Vec<f64>], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [Vec<f64>], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f64 {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}
