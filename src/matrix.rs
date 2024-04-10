use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut, SubAssign};
use std::rc::Rc;

use num::traits::real::Real;
use num::zero;

use crate::error::{CalculationError, ErrorReason};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    matrix: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

pub(crate) struct EliminationResult<T> where T: Real + SubAssign + AddAssign + Add {
    pub result: Matrix<T>,
    pub epsilon: Matrix<T>,
}

type Result<T> = std::result::Result<T, CalculationError>;

impl<T> Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    pub(crate) fn new(rows: usize, cols: usize) -> Self {
        let mut matrix: Vec<Vec<T>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row: Vec<T> = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(zero());
            }
            matrix.push(row);
        }
        Self { matrix, rows, cols }
    }
    pub(crate) fn new_column_matrix(size: usize) -> Self {
        Self::new(size, 1)
    }
    fn echelon(&mut self, row: usize, row_against: usize) -> Result<()> {
        if self[row][row] == zero() {
            return Err(CalculationError::new(ErrorReason::UnableToCalculate));
        }
        let factor = self[row_against + 1][row] / self[row][row];
        (row..self.rows + 1).for_each(|some_next_row| {
            let second_factor = self[row][some_next_row];
            self[row_against + 1][some_next_row] -= factor * second_factor;
        });
        Ok(())
    }

    fn eliminate(&mut self, i: usize) -> Result<()> {
        if self[i][i] == zero() {
            return Err(CalculationError::new(ErrorReason::UnableToCalculate));
        }
        for j in (1..i + 1).rev() {
            let factor = self[j - 1][i] / self[i][i];
            for k in (0..self.rows + 1).rev() {
                let second_factor = self[i][k];
                self[j - 1][k] -= factor * second_factor;
            }
        }
        Ok(())
    }
    pub(crate) fn rows(&self) -> usize {
        self.rows
    }
    pub(crate) fn cols(&self) -> usize {
        self.cols
    }
    pub(crate) fn calculate_right(&self, calculated_result: &Matrix<T>) -> Matrix<T> {
        let mut result: Matrix<T> = Matrix::new(self.cols() - 1, 1);
        for row_idx in 0..self.rows() {
            let mut accumulator = zero();
            let size_of_calculated_result = calculated_result.rows;
            for current_root_idx in 0..size_of_calculated_result {
                accumulator += calculated_result[current_root_idx][0] * self[row_idx][current_root_idx];
            }
            result[row_idx][0] = accumulator;
        }
        result
    }
    pub(crate) fn get_rhs(&self) -> Self {
        let mut rhs = Matrix::new_column_matrix(self.rows);
        for i in 0..self.rows {
            rhs[i][0] = self.matrix[i][self.cols - 1];
        }
        rhs
    }
    pub(crate) fn gaussian_elimination(&self) -> Result<EliminationResult<T>> {
        if self.cols - 1 != self.rows {
            return Err(CalculationError::new(ErrorReason::IncorrectSize));
        }
        let mut cloned_matrix = self.clone();
        let mut matrix = Rc::new(&mut cloned_matrix);
        // Переводим матрицу в треугольный вид (Row-Echelon form)
        for i in 0..self.rows - 1 {
            for j in i..self.rows - 1 {
                Rc::get_mut(&mut matrix).unwrap().echelon(i, j)?;
            }
        }

        // Обратный ход Гаусса
        for i in (1..self.rows).rev() {
            Rc::get_mut(&mut matrix).unwrap().eliminate(i)?;
        }

        // Записываем решения
        let mut result: Matrix<T> = Matrix::new(self.rows, 1);
        for i in 0..self.rows {
            result[i][0] = matrix[i][self.rows] / matrix[i][i];
        }
        let mut epsilon = self.get_rhs();
        epsilon -= self.calculate_right(&result);
        for idx in 0..epsilon.rows() {
            epsilon[idx][0] = epsilon[idx][0].abs();
        }
        Ok(EliminationResult {result, epsilon})
    }
}

#[macro_export]
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::matrix::Matrix;
            Matrix::new(0, 0)
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::matrix::Matrix;
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let mut matrix = Matrix::new(rows, cols);
            for row_idx in 0..rows {
                for col_idx in 0..cols {
                    matrix[row_idx][col_idx] = data_as_nested_array[row_idx][col_idx];
                }
            }
            matrix
        }
    }
}

impl<T> Index<usize> for Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        &self.matrix[row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        &mut self.matrix[row]
    }
}

impl<T> SubAssign for Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    fn sub_assign(&mut self, rhs: Self) {
        if self.cols != rhs.cols {
            panic!("Некорректное число столбцов вычитаемой матрицы!");
        }
        if self.rows != rhs.rows {
            panic!("Некорректное число строк вычитаемой матрицы!");
        }
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self[row_idx][col_idx] -= rhs[row_idx][col_idx];
            }
        }
    }
}

impl<T> Display for Matrix<T> where T: Real + SubAssign + AddAssign + Add + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row_idx in 0..self.rows {
            write!(f, "[")?;
            for col_idx in 0..self.cols {
                write!(f, "{:#}", self[row_idx][col_idx])?;
            }
            write!(f, "]")?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Clone for Matrix<T> where T: Real + SubAssign + AddAssign + Add {
    fn clone(&self) -> Self {
        let mut matrix = Matrix::new(self.rows, self.cols);
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx];
            }
        }
        matrix
    }
}