use super::base::World;

pub trait Relocalizer {
    type World: 'static + World;
}
