use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorReason {
    IncorrectSize,
    UnableToCalculate,
}

impl ErrorReason {
    pub fn to_string(&self) -> &str {
        match self {
            ErrorReason::IncorrectSize => "Неверный размер у матрицы. Он должен быть n - 1 строк и n столбцов!",
            ErrorReason::UnableToCalculate => "У данной матрицы нет решений!"
        }
    }
}

impl Display for ErrorReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct CalculationError {
    reason: ErrorReason,
}

impl CalculationError {
    pub fn new(error_reason: ErrorReason) -> Self {
        Self {
            reason: error_reason
        }
    }
}

impl Display for CalculationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}