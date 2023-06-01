fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn the_three_types() {
    println!("There are three types of Strings");
    println!("String Litterals - stack - immutable");
    let s_lit = "I am a String Litteral";
    let s_str = String::from("I am a String String");
    
    println!("s_lit is a: {}, with value: {}", type_of(&s_lit), s_lit);
    println!("s_lit is a: {}, with value: {}", type_of(&s_str), s_str);
    

}

