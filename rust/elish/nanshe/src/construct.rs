use num_traits::{Num, Unsigned};

pub trait Fill<Ix: Unsigned, Val: Num, const LEN: usize> {
    type Output;
    fn fill(dim: [Ix; LEN], v: Val) -> Self::Output;
}

pub trait Range<Ix: Unsigned, Val: Num, const LEN: usize> {
    type Output;
    fn range(dim: [Ix; LEN], start: Val, step: [Val; LEN]) -> Self::Output;
}
