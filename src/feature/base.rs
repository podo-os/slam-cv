use crate::number::Number;

pub trait Feature {
    type Number: 'static + Number;
}
