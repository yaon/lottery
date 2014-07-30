extern crate time;
use self::time::{Timespec};

#[deriving(Show)]
pub struct TransactionMeta {
  id_client:        u32,
  id_transaction:   u32,
  open_time:        Timespec,
  close_time:       Option<Timespec>,
  start_op_time:    Option<Timespec>,
  end_op_time:      Option<Timespec>
}

impl TransactionMeta {
  pub fn new(client: u32, trans:u32, open: Timespec) -> TransactionMeta {
    TransactionMeta {
      id_client: client, id_transaction: trans,
      open_time: open, close_time: None,
      start_op_time: None, end_op_time: None
    }
  }
}

#[deriving(Show)]
pub enum Command {
  Add(TransactionMeta, String, String),
  Get(TransactionMeta, String)
}

#[deriving(Show)]
pub enum Ack {
  Error(TransactionMeta, String),
  Value(TransactionMeta, String, String)
}

pub static SOCKET_PATH: &'static str = "socket-unix-test";
