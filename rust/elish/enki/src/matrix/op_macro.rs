macro_rules! forward_val_val_binop {
    (impl $Op: ident for $Res: ty, $method: ident) => {
        impl $Op for $Res {
            type Output = Self;
            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                $Op::$method(self, &other)
            }
        }
    };
}

macro_rules! forward_ref_val_binop {
    (impl $Op: ident for $Res: ty, $method: ident) => {
        impl $Op<$Res> for &$Res {
            type Output = $Res;
            #[inline]
            fn $method(self, other: $Res) -> Self::Output {
                $Op::$method(other, self)
            }
        }
    };
}
