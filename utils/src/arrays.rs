
// Return type of variable
pub fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}

use num_traits::{cast::FromPrimitive, Zero, zero};
use ndarray::{Array, array, Array2, Array3, ArrayBase, ArrayView2, ArrayView, Dimension, Ix3};
use std::ops::{Add, Div};

fn add_tuples(tup1: (usize, usize), tup2: (usize, usize)) -> (usize, usize) {
    (tup1.0+tup2.0,
    tup1.1+tup2.1)
}

fn sub_tuples(tup1: (usize, usize), tup2: (usize, usize)) -> (usize, usize) {
    (tup1.0-tup2.0,
    tup1.1-tup2.1)
}


fn creations() {
    let a = array![[1., 2., 4.], [11., 12., 23.,]];
    let b = Array::range(0., 10.,0.5);
    let c: Array<f32, _> = Array::ones((4,5,6));

    println!("{:?}", type_of(&a));
    println!("{:?}", type_of(&b));

}

// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
fn slicing() {
    let a = array![[1., 2., 3.], [11., 22., 33.,]];
    println!("{:?}", a[[0, 1]]);
}

fn scalare_fn<'a, T, D>(view: ArrayView<'a, T, D>) -> T 
where
    T: Clone + FromPrimitive + Add<Output = T> + Div<Output = T> + Zero,
    D: Dimension
{
    view.mean().unwrap_or( zero::<T>())
}




pub fn main() {
    println!("Creation");
    creations();
    println!("Slicing");
    slicing();
    let a: Array2<f32> = Array::ones((10,10));

    let window_dim = (2,3);
    let out_dim = add_tuples(sub_tuples(a.dim(), window_dim), (1,1));
    let c = Array::from_iter(a.windows(window_dim).into_iter().map(|w| scalare_fn(w))).into_shape(out_dim).unwrap();

    println!("{:?}", c);
    
}










