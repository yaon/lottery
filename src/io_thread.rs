use std::io::{Acceptor, TcpListener};
use std::comm::Select;
use block::Block;

pub struct IOThread/*<T>*/ {
  send: Sender<uint>,
  recv: Receiver<uint>,
  // acceptor: Box<Acceptor<T>>,
  select: Select
}

impl IOThread {
}

impl Block for IOThread {
  fn new(send: Sender<uint>, recv: Receiver<uint>) -> IOThread {
    let listener = TcpListener::bind("0.0.0.0", 3737);
    IOThread {
      send: send,
      recv: recv,
      // acceptor: listener.listen(),
      select: ::std::comm::Select::new()
    }
  }

  fn start(&self) -> () {
    println!("hello IOThread");
  }

  fn exit(&self) -> () {
    // drop(self.acceptor);
    // drop(self.select);
    println!("bye IOThread");
  }
}
