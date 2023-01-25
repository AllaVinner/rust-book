use image::io::Reader as ImageReader;
use image;

fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn main() {
    let img = image::open("blocks.jpg").expect("File not found!");
    //let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
    println!("{:?}", type_of(&img));
    //println!("{:?}", type_of::type_of(&img2));
}