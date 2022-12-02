#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}



fn main() {
    let r = Rectangle {
        width: 30.,
        height: 50.,
    };

    println!(" The area is {}", area(&r));
    println!("rect is {:#?}", r);
    dbg!(&r);
}

fn area(rectangle: &Rectangle) -> f32 {
    rectangle.height * rectangle.width
}
