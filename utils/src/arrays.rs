
// Return type of variable
pub fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}


use ndarray::{Array, array};



fn creations() {
    let a = array![[1., 2., 4.], [11., 12., 23.,]];
    let b = Array::range(0., 10.,0.5);
    println!("{:?}", type_of(&a));
    println!("{:?}", type_of(&b));

}

pub fn main() {
    creations();
}










