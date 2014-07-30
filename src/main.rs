#![feature(phase)]
#[phase(plugin, link)] extern crate log;
use utils::{ Block, Command, Ack };
use io_thread::IOThread;
use pp_thread::Worker;
use std::sync::{ Mutex, Arc, RWLock };
use bptree::Loto;

mod utils;
mod io_thread;
mod pp_thread;
mod bptree;

static NPROC : uint = 4;


fn main() {
  let (tpp_tx, tio_rx): (Sender<Ack>, Receiver<Ack>) = channel();
  let (tio_tx, tpp_rx): (Sender<Command>, Receiver<Command>) = channel();

  let tpp_rx_mutex = Arc::new(Mutex::new(tpp_rx));

  debug!("Spawning IO thread");
  spawn(proc() {
    let mut tio: IOThread = Block::new(tio_tx, tio_rx);
    tio.start();
  });

  let db_lock = Arc::new(RWLock::new(Loto::new(None)));

  for i in range(0u, NPROC) {
    let db_lock_clone = db_lock.clone();
    let tpp_rx_mutex_clone = tpp_rx_mutex.clone();
    let tpp_tx_clone = tpp_tx.clone();

    debug!("Spawning worker thread");
    spawn(proc() {
      let mut worker: Worker =
        Worker::new(i, tpp_tx_clone, tpp_rx_mutex_clone, db_lock_clone);
      worker.start();
    });
  }

  loop {
    std::io::timer::sleep(10000);
  }
}
