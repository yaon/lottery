// #![feature(phase)]
// #[phase(plugin, link)] extern crate log;
use std::sync::{Mutex, Arc};
use std::comm;

pub struct Worker {
  tx: Sender<String>,
  rx_mutex: Arc<Mutex<Receiver<String>>>,
}

impl Worker {
  pub fn new(tx: Sender<String>, rx_mutex: Arc<Mutex<Receiver<String>>>) -> Worker {
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
