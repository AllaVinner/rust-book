

// Defining Trait
trait MyTrait {
    // Declare Trait function
	fn fun1(&self) -> i32;
    // Initiate default function (Can be overwritten)
    fn fun_with_default(&self) -> i32 {
        42 as i32
    }
}

// Traits as parameters
fn trait_fun(a: & impl MyTrait) {
    a.fun1();
}

// Trait bound
fn bound_fun<T: MyTrait>(item: &T) {
    item.fun_with_default();
}

// Bound with same type
fn same_type_fun<T: MyTrait>(a: &T, b: &T){
    
}
