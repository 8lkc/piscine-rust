mod lalgebra_scalar;

pub use lalgebra_scalar::*;
pub use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> (pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {self.0[0].len()}
    pub fn number_of_rows(&self) -> usize {self.0.len()}

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {return self.0[n].clone()}
        Vec::new()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        if n >= self.number_of_cols() {return result}
        for i in 0..self.number_of_rows() {result.push(self.0[i][n].clone())}
        result
    }
}

impl<T> Mul for Matrix<T>
where T: Mul<Output = T> + Add<Output = T> + Clone + Default {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Option<Self> {
        if self.number_of_cols() != other.number_of_rows() {return None}
        let mut result: Vec<Vec<T>> = vec![vec![T::default(); other.number_of_cols()]; self.number_of_rows()];
        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {sum = sum + (self.0[i][k].clone() * other.0[k][j].clone())}
                result[i][j] = sum;
            }
        }
        Some(Matrix(result))
    }
}
