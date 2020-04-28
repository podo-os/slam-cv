use crate::feature::Landmark;

pub trait World {
    type Landmark: 'static + Landmark;

    fn for_landmarks<F>(&self, f: F)
    where
        F: FnMut(&Self::Landmark);

    #[cfg(feature = "serde")]
    fn load(&self);

    #[cfg(feature = "serde")]
    fn save(&self);
}
