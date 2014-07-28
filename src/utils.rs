#[deriving(Show)]
pub enum Command {
  Add(String, String),
  Del(String),
  Get(String)
}

#[deriving(Show)]
pub enum Ack {
  Error(String),
  Value(String, String),
}

pub trait Block {
  fn new(send: Sender<Command>, recv: Receiver<Ack>) -> Self;
  fn start(&self) -> ();
  fn exit(&self) -> ();
}

pub static SOCKET_PATH: &'static str = "socket-unix-test";

pub static NPROC : uint = 4u;

// Max number of keys per node
pub static ORDER: uint = 4;
