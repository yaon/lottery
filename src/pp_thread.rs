use utils::{Block, Command};

pub struct TPP {
  send: Sender<Command>,
  recv: Receiver<Command>
}

impl TPP {
}

impl Block for TPP {
  fn new(send: Sender<Command>, recv: Receiver<Command>) -> TPP {
    TPP { send: send, recv: recv }
  }

  fn start(&self) -> () {
    println!("hello TPP");
  }

  fn exit(&self) -> () {
    println!("bye TPP");
  }
}
