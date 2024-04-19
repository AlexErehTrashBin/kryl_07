pub mod matrix;
pub mod error;

use num::traits::real::Real;

fn main() {
    let matrix = matrix![
        0.43, 1.24, -0.58, 2.71;
        0.74, 0.83, 1.17, 1.26;
        1.43, -1.58, 0.83, 1.03
    ];
    match matrix.gaussian_elimination() {
        Err(e) => {
            println!("{e}")
        }
        Ok(result) => {
            println!("Найденные корни: ");
            println!("{:}", result.result);
            println!("Найденная невязка: ");
            let mut eps = matrix.calculate_right(&result.epsilon);
            eps.map_each(|x| {x.abs()});
            println!("{:}", eps)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;
    use crate::matrix::Matrix;

    #[test]
    fn test_gauss() {
        let matrix: Matrix<f32> = matrix![
            1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0;
            3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0;
            1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0;
            1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0;
            1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0;
            1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0
        ];
        let result = matrix![
            -264.05893; 159.63196; -6.156921; 35.310387; -18.806696; 81.67839
        ];
        assert_eq!(matrix.gaussian_elimination().unwrap().result, result);
    }
}