pub trait Solver {
    type Result: 'static;

    fn solve(self) -> Self::Result;
}
