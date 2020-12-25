use num_traits::{Num, Unsigned};

pub trait FillOne<Ix: Unsigned, Val: Num> {
    type Output;
    fn fill(n: Ix, v: Val) -> Self::Output;
}

pub trait FillTwo<Ix: Unsigned, Val: Num> {
    type Output;
    fn fill(nrow: Ix, ncol: Ix, v: Val) -> Self::Output;
}

pub trait Range<Ix: Unsigned, Val: Num> {
    type Output;
    fn range(n: Ix, start: Val, step: Val) -> Self::Output;
}
