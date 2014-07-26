pub trait Block {
  fn new(send: Sender<uint>, recv: Receiver<uint>) -> Self;
  fn start(&self) -> ();
  fn exit(&self) -> ();
}

pub static SOCKET_PATH: &'static str = "socket-unix-test";
