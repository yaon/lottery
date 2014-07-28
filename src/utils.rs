#[deriving(Show)]
pub enum Command {
  Add(String, String),
  Del(String),
  Get(String)
}

pub trait Block {
  fn new(send: Sender<Command>, recv: Receiver<Command>) -> Self;
  fn start(&self) -> ();
  fn exit(&self) -> ();
}

pub static SOCKET_PATH: &'static str = "socket-unix-test";
