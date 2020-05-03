use crate::feature::Landmark;
use crate::frame::KeyFrame;
use crate::number::Number;

pub trait World {
    type Number: 'static + Number;
    type Landmark: 'static + Landmark<Number = Self::Number>;
    type KeyFrame: 'static + KeyFrame<Number = Self::Number, Feature = Self::Landmark>;

    fn for_landmarks<F>(&self, f: F)
    where
        F: FnMut(&Self::Landmark);

    fn collect_landmarks<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&Self::Landmark) -> B;

    fn collect_keyframes<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&Self::KeyFrame) -> B;

    #[cfg(feature = "serde")]
    fn load(&self);

    #[cfg(feature = "serde")]
    fn save(&self);
}
