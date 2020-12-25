use num_traits::Unsigned;

use crate::dim::Dim;

pub trait Tensor<Ix: Unsigned, const LEN: usize>: Dim<Ix, LEN> {}

