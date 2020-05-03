pub struct Colors;

// Red, Green, Blue
type RgbColor = [f32; 3];

impl Colors {
    pub fn black() -> RgbColor {
        [0.0, 0.0, 0.0]
    }

    pub fn white() -> RgbColor {
        [1.0, 1.0, 1.0]
    }

    pub fn red() -> RgbColor {
        [1.0, 0.0, 0.0]
    }

    pub fn green() -> RgbColor {
        [0.0, 1.0, 0.0]
    }

    pub fn blue() -> RgbColor {
        [0.0, 0.0, 1.0]
    }
}
