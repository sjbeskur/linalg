#![allow(dead_code)]
use std::ops;

#[derive(Debug)]
pub struct Matrix<'a, T> {
    raw:  &'a [T],
}

impl<'a, T: PartialEq + Sized> Matrix<'a, T> {
    pub fn new(raw: &'a [T]) -> Matrix<'a, T>{
        Self{
            raw: &raw,
        }
    }
}

// addition / subtraction
// scalar multiplication / division
// identity
// is square 
// det - determinant of matrix
// inverse 
// orthogonal 


#[test]
pub fn test_matrix_new(){
    let m1 = Matrix::new(&[1.0,2.0,3.0]);
}