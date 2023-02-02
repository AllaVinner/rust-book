use std::option;


#[derive(Debug)]
pub struct Optional {
    pub arg2: i32,
    pub arg3: f32,
    _private: (),
 }
 
 impl Default for Optional {
     fn default() -> Self {
         Optional {arg2: 42,
         arg3: 0.3333333333,
         _private: (),
         }
     }
 }

 fn optional_fn(arg1: i32, optional_args: Optional) {
    println!("{:?} args", arg1);
    println!("{:?} args", optional_args);
 }

fn myfun(arg1: Option<f32>) {
    println!("{:?} IN fun", arg1);
}

pub fn main() {
    myfun(Some(33.));
    optional_fn(34, Optional { arg3: 12., ..Default::default()})
}




