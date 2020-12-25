use async_std::channel::{Receiver, Sender};
use log::{error, info};
use std::collections::hash_map::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::time::{Duration, Instant};

pub async fn man_loop(man_events: Receiver<ManEvent>, timeout: &Duration) {
    let mut conn_pool = ConnPool::<String>::new();
    loop {
        match man_events.recv().await {
            Ok(event) => match event {
                ManEvent::Est(man, wr_ch) => conn_pool.est(man, wr_ch),
                ManEvent::Com(man) => conn_pool.com(man),
                ManEvent::Expire => conn_pool.expire(timeout),
                ManEvent::Exit(man, msg) => conn_pool.exit(&man, &msg),
            },
            Err(err) => {
                error!("Management events channel error. At: {}", err);
                break;
            }
        }
    }
}

pub enum ManEvent {
    Est(String, Sender<Vec<u8>>),
    Com(String),
    Expire,
    Exit(String, String),
}

struct Conn {
    last: Instant,
    wr_ch: Sender<Vec<u8>>,
}

impl Conn {
    #[inline]
    fn new(wr_ch: Sender<Vec<u8>>) -> Conn {
        Conn {
            last: Instant::now(),
            wr_ch,
        }
    }
    #[inline]
    fn com(&mut self) {
        self.last = Instant::now();
    }
    #[inline]
    fn expired(&self, timeout: &Duration) -> bool {
        self.last.elapsed() >= *timeout
    }
}

struct ConnPool<Addr: Eq + Hash + Display> {
    pool: HashMap<Addr, Conn>,
}

impl<Addr: Eq + Hash + Display> ConnPool<Addr> {
    #[inline]
    fn new<Ad: Eq + Hash + Display>() -> ConnPool<Ad> {
        ConnPool {
            pool: HashMap::new(),
        }
    }
    #[inline]
    fn est(&mut self, man: Addr, wr_ch: Sender<Vec<u8>>) {
        info!("Connection is established: [{}]", &man);
        self.pool.insert(man, Conn::new(wr_ch));
    }
    #[inline]
    fn com(&mut self, man: Addr) {
        self.pool.entry(man).and_modify(|conn| conn.com());
    }
    #[inline]
    fn expire(&mut self, timeout: &Duration) {
        self.pool.retain(|man, conn| {
            if conn.expired(timeout) {
                error!("Connection is expired: [{}]", man);
                false
            } else {
                true
            }
        })
    }
    #[inline]
    fn exit(&mut self, man: &Addr, msg: &str) {
        if let Some(_) = self.pool.remove(man) {
            error!("Remote error: [{}]. At: {}", man, msg);
        }
    }
}
