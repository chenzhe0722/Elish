pub trait Trans {
    type Output;
    fn trans(self) -> Self::Output;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

pub trait GeneralInv {
    type Output;
    fn general_inv(self) -> Self::Output;
}

pub trait ApproxInv {
    type Output;
    fn approx_inv(self) -> Self::Output;
}
