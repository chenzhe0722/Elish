#[macro_use]
mod op_macro;
pub mod construct;

use nanshe::matrix::construct::FillTwo;
use nanshe::matrix::ops::Trans;
use nanshe::matrix::Dim;
use nanshe::matrix::Mat;
use num_bigint::BigUint;
use num_traits::Num;
use std::mem::swap;
use std::ops::{Add, Mul, Sub};

pub struct Matrix {
    n_row: BigUint,
    n_col: BigUint,
}

impl Mat<BigUint> for Matrix {}

impl Dim<BigUint> for Matrix {
    #[inline]
    fn nrow(&self) -> &BigUint {
        &self.n_row
    }
    #[inline]
    fn ncol(&self) -> &BigUint {
        &self.n_col
    }
}

impl Matrix {
    #[inline]
    fn debug_assert_add(&self, rhs: &Self) {
        debug_assert_eq!(self.nrow(), rhs.nrow());
        debug_assert_eq!(self.ncol(), rhs.ncol());
    }
    #[inline]
    fn debug_assert_mul(&self, rhs: &Self) {
        debug_assert_eq!(self.ncol(), rhs.nrow());
    }
}

impl<Val: Num> FillTwo<BigUint, Val> for Matrix {
    type Output = Matrix;
    fn fill(nrow: BigUint, ncol: BigUint, _: Val) -> Self::Output {
        Matrix {
            n_row: nrow,
            n_col: ncol,
        }
    }
}

impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        self.debug_assert_add(rhs);
        Matrix {
            n_row: self.nrow().clone(),
            n_col: self.ncol().clone(),
        }
    }
}

forward_ref_val_binop!(impl Add for Matrix, add);

impl Add<&Self> for Matrix {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        self.debug_assert_add(rhs);
        self
    }
}

forward_val_val_binop!(impl Add for Matrix, add);

impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        self.debug_assert_add(rhs);
        Matrix {
            n_row: self.nrow().clone(),
            n_col: self.ncol().clone(),
        }
    }
}

impl Sub<Matrix> for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output {
        self.debug_assert_add(&rhs);
        rhs
    }
}

impl Sub<&Self> for Matrix {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        self.debug_assert_add(rhs);
        self
    }
}

forward_val_val_binop!(impl Sub for Matrix, sub);

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        self.debug_assert_mul(rhs);
        Matrix {
            n_row: self.nrow().clone(),
            n_col: rhs.ncol().clone(),
        }
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, mut rhs: Matrix) -> Self::Output {
        self.debug_assert_mul(&rhs);
        rhs.n_row = self.nrow().clone();
        rhs
    }
}

impl Mul<&Self> for Matrix {
    type Output = Self;
    fn mul(mut self, rhs: &Self) -> Self::Output {
        self.debug_assert_mul(rhs);
        self.n_col = rhs.ncol().clone();
        self
    }
}

forward_val_val_binop!(impl Mul for Matrix, mul);

impl Trans for Matrix {
    type Output = Self;
    fn trans(mut self) -> Self::Output {
        swap(&mut self.n_row, &mut self.n_col);
        self
    }
}

impl Trans for &Matrix {
    type Output = Matrix;
    fn trans(self) -> Self::Output {
        Matrix {
            n_row: self.n_col.clone(),
            n_col: self.n_row.clone(),
        }
    }
}
