use async_std::channel::{Receiver, Sender};

pub fn ps_loop(ps_event: Receiver<PsEvent>) {}

pub enum PsEvent {}
