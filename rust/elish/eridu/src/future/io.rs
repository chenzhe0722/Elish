use futures_lite::{AsyncReadExt, AsyncWriteExt};
use std::io::Result;
use std::mem::size_of;

macro_rules! read_num {
    ($Ty: ty, $method: ident) => {
        pub async fn $method<T>(istream: &mut T) -> Result<$Ty>
        where
            T: AsyncReadExt + Unpin,
        {
            let mut buf = [0_u8; size_of::<$Ty>()];
            match istream.read_exact(&mut buf).await {
                Ok(()) => Ok(<$Ty>::from_ne_bytes(buf)),
                Err(err) => Err(err),
            }
        }
    };
}

macro_rules! write_num {
    ($Ty: ty, $method: ident) => {
        pub async fn $method<T>(ostream: &mut T, num: $Ty) -> Result<()>
        where
            T: AsyncWriteExt + Unpin,
        {
            ostream.write_all(&<$Ty>::to_ne_bytes(num)).await
        }
    };
}

macro_rules! rw_num {
    ($Ty: ty, $read: ident, $write: ident) => {
        read_num!($Ty, $read);
        write_num!($Ty, $write);
    };
}

rw_num!(u8, read_u8, write_u8);
rw_num!(u16, read_u16, write_u16);
rw_num!(u32, read_u32, write_u32);
rw_num!(u64, read_u64, write_u64);
rw_num!(u128, read_u128, write_u128);
rw_num!(i8, read_i8, write_i8);
rw_num!(i16, read_i16, write_i16);
rw_num!(i32, read_i32, write_i32);
rw_num!(i64, read_i64, write_i64);
rw_num!(i128, read_i128, write_i128);
rw_num!(f32, read_f32, write_f32);
rw_num!(f64, read_f64, write_f64);

macro_rules! read_sized {
    ($Size: ty, $method: ident) => {
        pub async fn $method<T>(istream: &mut T) -> Result<Vec<u8>>
        where
            T: AsyncReadExt + Unpin,
        {
            let mut head = [0_u8; size_of::<$Size>()];
            match istream.read_exact(&mut head).await {
                Ok(()) => {
                    let mut msg = vec![0_u8; <$Size>::from_ne_bytes(head) as usize];
                    match istream.read_exact(&mut msg).await {
                        Ok(()) => Ok(msg),
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
    };
}

macro_rules! write_sized {
    ($Size: ty, $method: ident) => {
        pub async fn $method<T>(ostream: &mut T, msg: &[u8], size: usize) -> Result<()>
        where
            T: AsyncWriteExt + Unpin,
        {
            debug_assert_eq!(size, size as $Size as usize);
            let head = <$Size>::to_ne_bytes(size as $Size);
            match ostream.write_all(&head).await {
                Ok(()) => ostream.write_all(msg).await,
                Err(err) => Err(err),
            }
        }
    };
}

macro_rules! rw_sized {
    ($Size: ty, $read_method: ident, $write_method: ident) => {
        read_sized!($Size, $read_method);
        write_sized!($Size, $write_method);
    };
}

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
rw_sized!(u8, read_sized_u8, write_sized_u8);

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
rw_sized!(u16, read_sized_u16, write_sized_u16);

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
rw_sized!(u32, read_sized_u32, write_sized_u32);

#[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
rw_sized!(u64, read_sized_u64, write_sized_u64);

#[cfg(any(target_pointer_width = "128"))]
rw_sized!(u128, read_sized_u128, write_sized_u128);
