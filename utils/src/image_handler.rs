use image::io::Reader as ImageReader;

fn type_of<T>(_:&T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn main() {
    let img = ImageReader::open("blocks.jpg")?.decode()?;
    //let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
    println!("{:?}", type_of(&img));
    //println!("{:?}", type_of::type_of(&img2));
}