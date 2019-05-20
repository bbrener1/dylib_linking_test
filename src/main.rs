#[macro_use(array)]
extern crate ndarray;
extern crate ndarray_linalg;
extern crate intel_mkl_src;

use ndarray::{Array,Ix2};
use ndarray_linalg::{Factorize,Inverse,SVD};

fn main() {

    let mut mtx: Array<f64,Ix2> = Array::from_shape_vec((2,2),vec![1.,2.,3.,4.]).unwrap();
    mtx.factorize().unwrap().inv();

    mtx = array![[1.,0.],[0.,1.]];
    mtx.svd(true,true);

    println!("Hello, world!");

}
