use matrix::*;

fn main() {
    let i_m:Matrix<u32> = Matrix::new();
    let f_m:Matrix<f32> = Matrix::new();
    println!("INTEGER MATRIX -> {:?}\nFLOAT MATRIX -> {:?}", i_m, f_m);
	println!("{:?}", Matrix::<f64>::zero(3, 4));
	println!("{:?}", Matrix::<i32>::identity(4));
}
