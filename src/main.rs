use std::time::Instant;

use eyre::Result;
use image::GrayImage;
use visr::equalize;

fn main() -> Result<()> {
    let file_name = std::env::args().nth(1).expect("No image path provided");
    let image = image::open(file_name).unwrap().to_luma8();
    image.save("luma.png")?;

    let t = Instant::now();
    let equalized = equalize(&image);
    let t = Instant::now() - t;
    eprintln!("Equalized in {t:?}");

    let equalized_image = GrayImage::from_vec(image.width(), image.height(), equalized)
        .expect("Could not generate the equalized image");

    equalized_image.save("equalized.png")?;

    Ok(())
}
