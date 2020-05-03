pub struct Colors;

// Red, Green, Blue
type RgbColor = [f32; 3];

impl Colors {
    pub const fn black() -> RgbColor {
        [0.0, 0.0, 0.0]
    }

    pub const fn white() -> RgbColor {
        [1.0, 1.0, 1.0]
    }

    pub const fn red() -> RgbColor {
        [1.0, 0.0, 0.0]
    }

    pub const fn green() -> RgbColor {
        [0.0, 1.0, 0.0]
    }

    pub const fn blue() -> RgbColor {
        [0.0, 0.0, 1.0]
    }
}
