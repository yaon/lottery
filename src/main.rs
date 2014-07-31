#![feature(phase)]
#[phase(plugin, link)] extern crate log;
use utils::{ Command, Ack};
use io_thread::{IThread, OThread, Client};
use worker::Worker;
use std::sync::{ Mutex, Arc, RWLock };
use db::DB;

mod utils;
mod io_thread;
mod worker;
mod db;

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

  let db_lock = Arc::new(RWLock::new(DB::new(None)));

  for i in range(0u, NPROC) {
    let db_lock_clone = db_lock.clone();
    let tpp_rx_mutex_clone = tpp_rx_mutex.clone();
    let tpp_tx_clone = tpp_tx.clone();

    debug!("Spawning worker thread");
    spawn(proc() {
      let worker: Worker =
        Worker::new(i, tpp_tx_clone, tpp_rx_mutex_clone, db_lock_clone);
      worker.start();
    });
  }

  loop {
    std::io::timer::sleep(10000);
  }
}
