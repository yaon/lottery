#![feature(phase)]
#[phase(plugin, link)] extern crate log;
use std::os;
use utils::{ Command, Ack};
use io_thread::{IThread, OThread, Client};
use worker::Worker;
use std::sync::{ Mutex, Arc, RWLock };
use db::DB;

mod utils;
mod io_thread;
mod worker;
mod db;

fn main() {
  let args = os::args();

  let nproc =
    if args.len() == 2 {
      match from_str(args.get(1).as_slice().trim()) {
                Some(n) => n,
                None    => 4
        }
    } else {
      4
    };

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

  for i in range(0u, nproc) {
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
