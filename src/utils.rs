extern crate time;
use self::time::{Timespec, get_time};

#[deriving(Show, Clone)]
pub struct TransactionMeta {
  pub id_client:        uint,
  pub id_transaction:   uint,
  pub open_time:        Timespec,
  pub close_time:       Option<Timespec>,
  pub start_op_time:    Option<Timespec>,
  pub end_op_time:      Option<Timespec>
}

#[deriving(Show, Clone)]
pub enum Command {
  Add(TransactionMeta, String, String),
  Get(TransactionMeta, String)
}

#[deriving(Show, Clone)]
pub enum Ack {
  Error(TransactionMeta, String),
  Value(TransactionMeta, String, String)
}

impl Ack {
  pub fn meta(&self) -> TransactionMeta {
    match *self {
      Error(meta, _) => meta,
      Value(meta, _, _) => meta
    }
  }

  pub fn update_start_op_time(&mut self) -> () {
    self.meta().update_start_op_time()
  }

  pub fn update_end_op_time(&mut self) -> () {
    self.meta().update_end_op_time()
  }

  pub fn update_close_time(&mut self) -> () {
    self.meta().update_close_time()
  }
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

pub static SOCKET_PATH: &'static str = "socket-unix-test";
