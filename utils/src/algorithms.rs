



pub fn inverse_fn(fun: Fn(f32) -> f32 , target_value: f32, min_value: f32, max_value: f32, num_iterations: i32) -> f32 {
    let mut guess =  (max_value - min_value) /2;
    let mut ceiling = max_value;
    let mut floor = min_value;
    for _ in 0..num_iterations {
        let value = fun(guess);
        if value > target_value {
            ceiling = guess;
        } else {
            floor = guess;
        }
        guess = (ceiling - floor)/2;
    }
    return guess
}

pub main()





