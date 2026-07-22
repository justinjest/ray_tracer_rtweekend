use crate::rtw_image::*;
use crate::rtweekend::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Copy, Clone)]
pub struct SolidColor {
    u: f64,
    v: f64,
    albedo: Color,
    p: Point3,
}

impl SolidColor {
    pub fn new(color: impl Into<Color>) -> Self {
        Self {
            u: 0.0,
            v: 0.0,
            albedo: color.into(),
            p: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

impl From<SolidColor> for Arc<dyn Texture> {
    fn from(tex: SolidColor) -> Self {
        Arc::new(tex)
    }
}

impl From<Color> for Arc<dyn Texture> {
    fn from(color: Color) -> Self {
        Arc::new(SolidColor::new(color))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(
        scale: f64,
        even: impl Into<Arc<dyn Texture>>,
        odd: impl Into<Arc<dyn Texture>>,
    ) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: even.into(),
            odd: odd.into(),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let scale = self.inv_scale;
        let x_int = (scale * p.x()).floor() as i32;
        let y_int = (scale * p.y()).floor() as i32;
        let z_int = (scale * p.z()).floor() as i32;

        let is_even = (x_int + y_int + z_int).rem_euclid(2) == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl From<Checker> for Arc<dyn Texture> {
    fn from(tex: Checker) -> Self {
        Arc::new(tex)
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl From<ImageTexture> for Arc<dyn Texture> {
    fn from(tex: ImageTexture) -> Self {
        Arc::new(tex)
    }
}

impl ImageTexture {
    pub fn new(file_name: &str) -> Self {
        Self {
            image: RtwImage::new(file_name),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let new_u = Interval::new(0.0, 1.0).clamp(u);
        let new_v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (new_u * (self.image.width() - 1) as f64) as u32;
        let j = (new_v * (self.image.height() - 1) as f64) as u32;
        self.image.pixel_data(i, j)
    }
}
