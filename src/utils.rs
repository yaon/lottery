extern crate time;
use self::time::{Timespec, get_time};

#[deriving(Show)]
pub struct TransactionMeta {
  id_client:        uint,
  id_transaction:   uint,
  open_time:        Timespec,
  close_time:       Option<Timespec>,
  start_op_time:    Option<Timespec>,
  end_op_time:      Option<Timespec>
}

impl TransactionMeta {
  pub fn new(client: uint, trans: uint, open: Timespec) -> TransactionMeta {
    TransactionMeta {
      id_client: client, id_transaction: trans,
      open_time: open, close_time: None,
      start_op_time: None, end_op_time: None
    }
  }
  pub fn update_start_op_time(&mut self) -> () {
    self.start_op_time = Some(get_time());
  }

  pub fn update_end_op_time(&mut self) -> () {
    self.end_op_time = Some(get_time());
  }

  pub fn update_close_time(&mut self) -> () {
    self.close_time = Some(get_time());
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
