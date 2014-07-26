use utils::Block;

pub struct TPP {
  send: Sender<uint>,
  recv: Receiver<uint>
}

impl TPP {
}

impl Block for TPP {
  fn new(send: Sender<uint>, recv: Receiver<uint>) -> TPP {
    TPP { send: send, recv: recv }
  }

  fn start(&self) -> () {
    println!("hello TPP");
  }

  fn exit(&self) -> () {
    println!("bye TPP");
  }
}
