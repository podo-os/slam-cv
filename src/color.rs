pub struct Colors;

// Red, Green, Blue
type RgbColor = nalgebra::Point3<f32>;

impl Colors {
    pub fn red() -> RgbColor {
        RgbColor::new(1.0, 0.0, 0.0)
    }

    pub fn white() -> RgbColor {
        RgbColor::new(1.0, 1.0, 1.0)
    }
}
