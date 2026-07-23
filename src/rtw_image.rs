use crate::rtweekend::*;
use image::{ImageReader, RgbImage};
use std::path::Path;

pub struct RtwImage {
    data: Option<RgbImage>,
}

impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let search_paths = [
            filename.to_string(),
            format!("images/{}", filename).to_string(),
            format!("../images/{}", filename).to_string(),
            format!("../../images/{}", filename).to_string(),
        ];

        for path in &search_paths {
            if Path::new(path).exists()
                && let Ok(reader) = ImageReader::open(path)
                && let Ok(decoded) = reader.decode()
            {
                return Self {
                    data: Some(decoded.into_rgb8()),
                };
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.", filename);
        Self { data: None }
    }

    pub fn width(&self) -> u64 {
        self.data.as_ref().map_or(0, |img| (img.width()).into())
    }

    pub fn height(&self) -> u64 {
        self.data.as_ref().map_or(0, |img| (img.height()).into())
    }

    pub fn pixel_data(&self, mut x: u32, mut y: u32) -> Color {
        let magenta: Color = Color::new(1.0, 0.0, 1.0);

        let Some(img) = &self.data else {
            return magenta;
        };

        x = x.clamp(0, img.width().saturating_sub(1));
        y = y.clamp(0, img.height().saturating_sub(1));

        let pixel = img.get_pixel(x, y);
        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
