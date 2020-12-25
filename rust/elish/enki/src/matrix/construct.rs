use crate::matrix::Matrix;
use nanshe::matrix::construct::{FillOne, Range};
use num_bigint::BigUint;
use num_traits::{Num, One};

pub struct RowVec;

impl<Val: Num> FillOne<BigUint, Val> for RowVec {
    type Output = Matrix;
    fn fill(n: BigUint, _: Val) -> Self::Output {
        Matrix {
            n_row: One::one(),
            n_col: n,
        }
    }
}

impl<Val: Num> Range<BigUint, Val> for RowVec {
    type Output = Matrix;
    fn range(n: BigUint, _: Val, _: Val) -> Self::Output {
        Matrix {
            n_row: One::one(),
            n_col: n,
        }
    }
}

pub struct ColVec;

impl<Val: Num> FillOne<BigUint, Val> for ColVec {
    type Output = Matrix;
    fn fill(n: BigUint, _: Val) -> Self::Output {
        Matrix {
            n_row: n,
            n_col: One::one(),
        }
    }
}

impl<Val: Num> Range<BigUint, Val> for ColVec {
    type Output = Matrix;
    fn range(n: BigUint, _: Val, _: Val) -> Self::Output {
        Matrix {
            n_row: n,
            n_col: One::one(),
        }
    }
}

pub struct Diag;

impl<Val: Num> FillOne<BigUint, Val> for Diag {
    type Output = Matrix;
    fn fill(n: BigUint, _: Val) -> Self::Output {
        Matrix {
            n_row: n.clone(),
            n_col: n,
        }
    }
}

impl<Val: Num> Range<BigUint, Val> for Diag {
    type Output = Matrix;
    fn range(n: BigUint, _: Val, _: Val) -> Self::Output {
        Matrix {
            n_row: n.clone(),
            n_col: n,
        }
    }
}
