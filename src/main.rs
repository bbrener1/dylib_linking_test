extern crate ndarray;
extern crate ndarray_linalg;
// extern crate blas;
// extern crate netlib_src;
extern crate openblas_src;
// extern crate intel_mkl_src;

use ndarray::{Array,Ix2};
use ndarray_linalg::{Factorize,Inverse};

fn main() {

    let mut mtx: Array<f64,Ix2> = Array::from_shape_vec((2,2),vec![1.,2.,3.,4.]).unwrap();
    mtx.factorize().unwrap().inv();

    println!("Hello, world!");

}
