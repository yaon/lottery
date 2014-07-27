#[deriving(Show)]
pub enum Command {
  Add(String, String),
  Del(String),
  Get(String)
}

impl Command {
  pub fn add(a:String, b:String) -> Command { Add(a, b) }
  pub fn del(a:String) -> Command { Del(a) }
  pub fn get(a:String) -> Command { Get(a) }
}

pub trait Block {
  fn new(send: Sender<Command>, recv: Receiver<Command>) -> Self;
  fn start(&self) -> ();
  fn exit(&self) -> ();
}

pub static SOCKET_PATH: &'static str = "socket-unix-test";
