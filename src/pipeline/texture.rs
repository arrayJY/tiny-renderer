use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};

use crate::Color;
pub struct Texture {
    image: DynamicImage,
    width: f32,
    height: f32,
}

impl Texture {
    pub fn from_path(path: &str) -> Result<Self, ()> {
        match ImageReader::open(path) {
            Ok(image_buffer) => {
                if let Ok(image) = image_buffer.decode() {
                    let (width, height) = image.dimensions();
                    Ok(Texture {
                        image,
                        width: width as f32,
                        height: height as f32,
                    })
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
    pub fn get(&self, u: f32, v: f32) -> Color {
        let x = u * self.width - 1.0;
        let y = self.height - 1.0 - v * self.height;
        let c = self.image.get_pixel(x as u32, y as u32);
        Color::rgba(c[0], c[1], c[2], c[3])
    }
}
