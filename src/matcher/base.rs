use crate::feature::{KeyPoint, Landmark};
use crate::number::Number;

pub trait Matcher {
    type KeyPoint: 'static + KeyPoint;
}

pub struct Match<KP>
where
    KP: KeyPoint,
{
    pub old: KP,
    pub new: KP,
}

pub struct WorldMatch<N, KP, LM>
where
    N: 'static + Number,
    KP: KeyPoint<Number = N>,
    LM: Landmark<Number = N>,
{
    pub old: KP,
    pub new: LM,
}
