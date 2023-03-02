use std::fs::File;

use tiff::decoder::DecodingResult;

fn main() {
    let ifs = match File::open("P1000017.tiff") {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };
    let mut tiff_decorder = match tiff::decoder::Decoder::new(ifs) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(decoder) => decoder,
    };
    let (width, height) = tiff_decorder.dimensions().unwrap();
    println!("{},{}", width, height);

    let img = tiff_decorder.read_image().unwrap();

    let pixels: Vec<u8> = if let DecodingResult::U8(pixels) = img {
        let mut output_pixels = pixels.clone();
        for (index, pixel) in pixels.iter().enumerate() {
            if index % 4 == 3 {
                continue;
            }
            output_pixels[index] = 255 - pixel;
        }
        output_pixels
    } else {
        panic!("wrong byte");
    };

    let mut file = File::create("output.tiff").unwrap();

    let mut tiff = tiff::encoder::TiffEncoder::new(&mut file).unwrap();
    tiff.write_image::<tiff::encoder::colortype::RGBA8>(width, height, &pixels);
}
