use num_traits::Unsigned;

pub trait Dim<Ix: Unsigned, const LEN: usize> {
    const NDIM: usize = LEN;
    fn dim(&self) -> &[Ix; LEN];
}
