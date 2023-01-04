

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
    a.fun1();
    b.fun_with_default();
}

//Multiple Traits
fn mult_fun<T: MyTrait + Add>(a: &T) {
    a.fun1();
}

// With where clause
fn where_fun<T>(a: &T) 
where 
    T: MyTrait + Clone
{
    a.fun1();
}

// Traits as return Type
fn ret_trait() -> impl MyTrait {}

// Only works for a single return type (Can't return different types from an if-clause).

// Trait Bounds to condition Implement methods
// I.E. T must implement Display and PartialOrd
// to be allowed to be used in paired.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations 
// Trait implementations on a type that is required to implement another trait
impl<T: Display> ToString for T {}

