use super::base::Feature;

pub trait Descriptor
where
    Self: Feature,
{
    type Distance: 'static;

    fn get_distance(&self, other: &Self) -> Self::Distance;
}
