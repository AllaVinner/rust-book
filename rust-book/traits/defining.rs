

trait MyTrait {
	fn fun1(&self) -> i32;
    fn fun_with_default(&self) -> i32 {
        42 as i32
    }
}


fn trait_fun(a: & impl MyTrait) {
    a.fun1();
}

