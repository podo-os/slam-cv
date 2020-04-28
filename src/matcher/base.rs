use crate::feature::{KeyPoint, Landmark};

pub trait Matcher {
    type KeyPoint: 'static + KeyPoint;
}

pub struct Match<KP>
where
    KP: 'static + KeyPoint,
{
    pub old: KP,
    pub new: KP,
}

pub struct WorldMatch<KP, LM>
where
    KP: 'static + KeyPoint,
    LM: 'static + Landmark,
{
    pub old: KP,
    pub new: LM,
}
