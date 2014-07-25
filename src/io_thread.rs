mod IOThread{
  use std::io::TcpListener;
  use std::io::{Listener, Acceptor, TimeOut};

  struct IOThread {
    acceptor: std::io::Acceptor,
    select: std::comm::Select
  }

  impl IOThread {

    fn new(&self) -> IOThread {
      let listener  = TcpListener::bind("0.0.0.0", "3737");
      IOThread {
        acceptor: listener.listen(),
        select:   std::comm::Select::new()
      }
    }

  }


  impl Function for IOThread {
    pub fn exit() -> () {
      drop(acceptor);
      drop(select);
    }

    pub fn start(send: Sender<T>, recv: Receiver<T>) -> () {
      IOThread::new();
    }
  }
}
