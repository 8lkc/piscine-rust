mod lalgebra_scalar;

pub use lalgebra_scalar::*;
pub use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> (pub Vec<Vec<T>>);

impl <T:Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        let mut two_dimensional_vector:Vec<Vec<T>> = Vec::new();
        let mut row:Vec<T> = Vec::new();
        row.push(T::zero()); two_dimensional_vector.push(row);
        Matrix(two_dimensional_vector)
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut two_dimensional_vector:Vec<Vec<T>> = Vec::new();
        for _ in 0..row {
            let mut row_i: Vec<T> = Vec::new();
            for _ in 0..col {row_i.push(T::zero())}
            two_dimensional_vector.push(row_i);
        }
        Matrix(two_dimensional_vector)
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut two_dimensional_vector:Vec<Vec<T>> = Vec::new();
        for i in 0..n {
            let mut row_i: Vec<T> = Vec::new();
            for j in 0..n {if i == j {row_i.push(T::one())} else {row_i.push(T::zero())}}
            two_dimensional_vector.push(row_i);
        }
        Matrix(two_dimensional_vector)
	}
}

impl<T: Scalar<Item = T> + Add<Output = T> + Clone> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {return None}
        let mut result = Vec::new();
        for (row_self, row_other) in self.0.iter().zip(other.0.iter()) {
            let mut new_row = Vec::new();
            for (elem_self, elem_other) in row_self.iter().zip(row_other.iter()) {new_row.push(elem_self.clone() + elem_other.clone())}
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T> + Clone> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {return None}
        let mut result = Vec::new();
        for (row_self, row_other) in self.0.iter().zip(other.0.iter()) {
            let mut new_row = Vec::new();
            for (elem_self, elem_other) in row_self.iter().zip(row_other.iter()) {new_row.push(elem_self.clone() - elem_other.clone())}
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}
