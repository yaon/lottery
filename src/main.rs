#![feature(phase)]
#[phase(plugin, link)] extern crate log;
use utils::{ Command, Ack};
use io_thread::{IThread, OThread, Client};
use pp_thread::Worker;
use std::sync::{ Mutex, Arc };

mod utils;
mod io_thread;
mod pp_thread;
mod bptree;

static NPROC : uint = 4;

fn main() {
  let (tpp_tx, tio_rx): (Sender<Ack>, Receiver<Ack>) = channel();
  let (tio_tx, tpp_rx): (Sender<Command>, Receiver<Command>) = channel();
  let (client_send, client_recv): (Sender<Client>, Receiver<Client>) = channel();

  let tpp_rx_mutex = Arc::new(Mutex::new(tpp_rx));

  debug!("Spawning IO thread");
  spawn(proc() {
    let mut ti: IThread = IThread::new(tio_tx, client_send);
    ti.start();
  });

  spawn(proc() {
    let mut to: OThread = OThread::new(client_recv, tio_rx);
    to.start();
  });

  for _ in range(0u, NPROC) {
    let tpp_rx_mutex_clone = tpp_rx_mutex.clone();
    let tpp_tx_clone = tpp_tx.clone();

    debug!("Spawning worker thread");
    spawn(proc() {
      let mut worker: Worker = Worker::new(tpp_tx_clone, tpp_rx_mutex_clone);
      worker.start();
    });
  }

  loop {
    std::io::timer::sleep(10000);
  }
}
