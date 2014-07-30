use std::sync::{ Mutex, Arc, RWLock };
use utils::{ Ack, Command, Add, Get };
use bptree::Loto;

pub struct Worker {
  id: uint,
  tx: Sender<Ack>,
  rx_mutex: Arc<Mutex<Receiver<Command>>>,
  db_lock: Arc<RWLock<Loto>>
}

impl Worker {
  pub fn new(
    id: uint,
    tx: Sender<Ack>,
    rx_mutex: Arc<Mutex<Receiver<Command>>>,
    db_lock: Arc<RWLock<Loto>>
  ) -> Worker {
    Worker { id: id, tx: tx, rx_mutex: rx_mutex, db_lock: db_lock }
  }
  pub fn start(self) {
    loop {
      let cmd = {
        let mut rx = self.rx_mutex.lock();
        debug!("Worker{}: Got mutex", self.id);
        let cmd = rx.recv();
        debug!("Worker{}: Received {}", self.id, cmd);
        cmd
      };
      let ack = match cmd {
        Add(k, v, m) => {
          let mut db = self.db_lock.write();
          db.command({ Add(k, v, m) })
        },
        Get(k, m) => {
          let mut db = self.db_lock.write(); // Read here
          db.command({ Get(k, m) })
        }
      };

      debug!("Worker {}: Ack: {}", self.id, ack);
      self.tx.send(ack);
    }
  }
}
