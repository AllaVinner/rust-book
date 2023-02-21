
// Return type of variable
pub fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}

use num_traits::{cast::FromPrimitive, Zero};
use ndarray::{Array, array, Array2, Array3, ArrayBase};
use std::ops::{Add, Div};


fn creations() {
    let a = array![[1., 2., 4.], [11., 12., 23.,]];
    let b = Array::range(0., 10.,0.5);
    let c: Array3<f32> = Array::ones((4,5,6));

    println!("{:?}", type_of(&a));
    println!("{:?}", type_of(&b));

}

// https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
fn slicing() {
    let a = array![[1., 2., 3.], [11., 22., 33.,]];
    println!("{:?}", a[[0, 1]]);
}

fn scalare_fn0(view: Array2<f32>) -> f32 {
    view.mean().unwrap()
}


fn scalare_fng<T>(view: Array2<T>) -> T 
where
    T: Clone + FromPrimitive + Add<Output = T> + Div<Output = T> + Zero 
{
    view.mean().unwrap()
}


pub fn main() {
    println!("Creation");
    creations();
    println!("Slicing");
    slicing();
    let a = array![[1., 2., 4.], [11., 12., 23.,]];
    let m0 = scalare_fn0(a.clone());
    let mg = scalare_fng(a);
    println!("Mean is {} {}", m0, mg);

    
}










