#![feature(phase)]
#[phase(plugin, link)] extern crate log;
use utils::Block;
use io_thread::IOThread;
use pp_thread::Worker;
use std::sync::{Mutex, Arc};

mod utils;
mod io_thread;
mod pp_thread;

static nprocs : uint = 4u;

fn main() {
  let (tpp_tx, tio_rx): (Sender<String>, Receiver<String>) = channel();
  let (tio_tx, tpp_rx): (Sender<String>, Receiver<String>) = channel();

  let tpp_rx_mutex = Arc::new(Mutex::new(tpp_rx));

  debug!("Spawning IO thread");
  spawn(proc() {
    let tio: IOThread = Block::new(tio_tx, tio_rx);
    tio.start();
  });

  for _ in range(0u, nprocs) {
    let tpp_rx_mutex_clone = tpp_rx_mutex.clone();
    let tpp_tx_clone = tpp_tx.clone();

    debug!("Spawning worker thread");
    spawn(proc() {
      let worker: Worker = Worker::new(tpp_tx_clone, tpp_rx_mutex_clone);
      worker.start();
    });
  }

  loop {
    std::io::timer::sleep(1000);
  }
}
