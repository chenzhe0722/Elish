pub mod construct;
pub mod ops;

use crate::matrix::ops::Trans;
use num_traits::Unsigned;
use std::ops::{Add, Mul, Sub};

pub trait Mat<Ix: Unsigned>: Dim<Ix> + MatOps + Sized {}

pub trait Dim<Ix: Unsigned> {
    fn nrow(&self) -> &Ix;
    fn ncol(&self) -> &Ix;
}

pub trait MatOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Trans<Output = Output>
{
}
impl<T, Rhs, Output> MatOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Trans<Output = Output>
{
}

pub trait MatRef<Ix: Unsigned>: Mat<Ix> + for<'r> MatOps<&'r Self> {}
impl<T, Ix: Unsigned> MatRef<Ix> for T where T: Mat<Ix> + for<'r> MatOps<&'r T> {}

pub trait RefMat<Base>: MatOps<Base, Base> + for<'r> MatOps<&'r Base, Base> {}
impl<T, Base> RefMat<Base> for T where T: MatOps<Base, Base> + for<'r> MatOps<&'r Base, Base> {}
