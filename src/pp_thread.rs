use std::sync::{ Mutex, Arc };
use utils::{ Ack, Command };
use bptree::command;

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
      let mut cmd;
      {
        let mut rx = self.rx_mutex.lock();
        debug!("Worker: Got mutex");
        cmd = rx.recv();
        debug!("Worker: Received {}", cmd);
      }
      // debug!("Workermsg: Sending {}", msg);
      self.tx.send(command(cmd));
    }
  }
}
