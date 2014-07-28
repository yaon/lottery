use std::sync::{Mutex, Arc};
use utils::{ Ack, Command };

pub struct Worker {
  tx: Sender<Ack>,
  rx_mutex: Arc<Mutex<Receiver<Command>>>,
}

impl Worker {
  pub fn new(tx: Sender<Ack>, rx_mutex: Arc<Mutex<Receiver<Command>>>) -> Worker {
    Worker { tx: tx, rx_mutex: rx_mutex }
  }
  pub fn start(&self) {
    loop {
      let mut msg;
      {
        let mut rx = self.rx_mutex.lock();
        debug!("Worker: Got mutex");
        msg = rx.recv();
        debug!("Worker: Received {}", msg);
      }
      // std::io::timer::sleep(1000);
      debug!("Workermsg: Sending {}", msg);
      self.tx.send(Ack::value(String::from_str("toto")));
    }
  }
}
