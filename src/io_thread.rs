
use std::io::net::unix::UnixListener;
use std::io::{fs,Acceptor,Listener};

use utils::SOCKET_PATH;
use utils::Block;

pub struct IOThread/*<T>*/ {
  send: Sender<uint>,
  recv: Receiver<uint>,
  socket: Path,
  // acceptor: Box<Acceptor<T>>,
  // select: Select
}

impl IOThread {
  fn unlink(&self) -> () {
    if self.socket.exists() {
      fs::unlink(&self.socket).unwrap();
    }
  }
}

impl Block for IOThread {
  fn new(send: Sender<uint>, recv: Receiver<uint>) -> IOThread {
    // let listener = TcpListener::bind("0.0.0.0", 3737);
    IOThread {
      send: send,
      recv: recv,
      socket: Path::new(SOCKET_PATH)
      // acceptor: listener.listen(),
      // select: ::std::comm::Select::new()
    }
  }

  fn start(&self) -> () {
    println!("hello IOThread");

    self.unlink();

    let stream = match UnixListener::bind(&self.socket) {
      Err(why) => fail!("failed to bind socket: {}", why),
      Ok(stream) => stream,
    };

    for mut client in stream.listen().incoming() {
      println!("IOThread: {}", client.read_to_str().unwrap());
      ::std::io::timer::sleep(1000);
    }

  }

  fn exit(&self) -> () {
    // drop(self.acceptor);
    // drop(self.select);
    self.unlink();
    println!("bye IOThread");
  }
}

