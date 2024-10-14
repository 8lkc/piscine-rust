mod lalgebra_scalar;
pub use lalgebra_scalar::*;

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

#[test]
fn zero_property() {
    let matrix: Matrix<u32> = Matrix::zero(3, 4);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::zero(2, 2);
    let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(matrix, expected);
}

#[test]
fn identity_matrix() {
    let matrix: Matrix<u32> = Matrix::identity(2);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
    assert_eq!(matrix, expected);

    let matrix: Matrix<u32> = Matrix::identity(3);
    let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(matrix, expected);
}