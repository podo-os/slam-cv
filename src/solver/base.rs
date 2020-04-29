pub trait Solver {
    type Output: 'static;

    fn solve(self) -> Self::Output;
}
