#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use std::sync::{Mutex, Arc};
mod test;


static nprocs : uint = 4u;

struct Worker {
  tx: Sender<int>,
  rx_mutex: Arc<Mutex<Receiver<int>>>,
}

impl Worker {
  fn new(tx: Sender<int>, rx_mutex: Arc<Mutex<Receiver<int>>>) -> Worker {
    Worker { tx: tx, rx_mutex: rx_mutex }
  }
  fn start(&self) {
    loop {
      let mut msg;
      {
        let mut rx = self.rx_mutex.lock();
        debug!("Worker: Got mutex");
        msg = rx.recv();
        debug!("Worker: Received {}", msg);
      }
      long_computation();
      debug!("Worker: Sending {}", msg);
      self.tx.send(msg);
    }
  }
}

fn main() {
  let (worker_tx, io_rx): (Sender<int>, Receiver<int>) = channel();
  let (io_tx, worker_rx): (Sender<int>, Receiver<int>) = channel();
  let worker_rx_mutex = Arc::new(Mutex::new(worker_rx));

  for _ in range(0u, nprocs) {
    let worker_rx_mutex_clone = worker_rx_mutex.clone();
    let worker_tx_clone = worker_tx.clone();

    debug!("Spawning worker thread");
    spawn(proc() {
      let worker: Worker = Worker::new(worker_tx_clone, worker_rx_mutex_clone);
      worker.start();
    });
  }

  debug!("Spawning tester thread");
  let tester: test::Tester = test::Tester::new(io_tx, io_rx);
  tester.start();

  0;
}

pub fn long_computation()
{
  std::io::timer::sleep(1000); // Long computation
}
