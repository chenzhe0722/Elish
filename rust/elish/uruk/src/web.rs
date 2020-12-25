pub mod tcp;

pub const LOCAL_HOST: &str = "localhost";
pub const LOCAL_IP: &str = "127.0.0.1";

#[inline]
pub fn domain(host: &str, port: u16) -> String {
    format!("{}:{}", host, port)
}
