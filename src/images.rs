use image::{DynamicImage, ImageFormat};
use image::io::Reader as ImageReader;
use std::io::Cursor;

#[derive(Debug)]
pub struct Images {
    pub air: &'static [u8],
    pub water: &'static [u8],
    pub earth: &'static [u8],
    pub fire: &'static [u8],
}

impl Images {
    pub fn new() -> Self {
        Images {
            air: include_bytes!("../images/air.png"),
            water: include_bytes!("../images/water.png"),
            earth: include_bytes!("../images/earth.png"),
            fire: include_bytes!("../images/fire.png"),
        }
    }
    pub fn fetch(&self, nation: &str) -> DynamicImage {
        let cursor = match nation.to_lowercase().as_str() {
            "air" => Cursor::new(self.air),
            "water" => Cursor::new(self.water),
            "earth" => Cursor::new(self.earth),
            "fire" => Cursor::new(self.fire),
            _ => Cursor::new(self.air)
        };
        let reader = ImageReader::with_format(cursor, ImageFormat::Png);
        let image = reader.decode().expect("Unable to get image");
        image
    }
}
