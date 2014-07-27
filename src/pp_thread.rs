use std::sync::{Mutex, Arc};
use utils::Command;

pub struct Worker {
  tx: Sender<Command>,
  rx_mutex: Arc<Mutex<Receiver<Command>>>,
}

impl Worker {
  pub fn new(tx: Sender<Command>, rx_mutex: Arc<Mutex<Receiver<Command>>>) -> Worker {
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
      debug!("Worker: Sending {}", msg);
      self.tx.send(msg);
    }
  }
}
