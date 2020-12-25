pub mod exec;
pub mod man;
pub mod ps;

#[repr(u8)]
pub enum Role {
    Man,
    Ps,
    Exec,
}
