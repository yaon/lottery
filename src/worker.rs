use std::sync::{ Mutex, Arc, RWLock };
use utils::{ Ack, Command, Add, Get };
use db::DB;

pub struct Worker {
  id: uint,
  tx: Sender<Ack>,
  rx_mutex: Arc<Mutex<Receiver<Command>>>,
  db_lock: Arc<RWLock<DB>>
}

impl Worker {
  pub fn new(
    id: uint,
    tx: Sender<Ack>,
    rx_mutex: Arc<Mutex<Receiver<Command>>>,
    db_lock: Arc<RWLock<DB>>
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
        Add(m, k, v) => {
          let mut db = self.db_lock.write();
          db.add(m, k, v)
        },
        Get(m, k) => {
          let mut db = self.db_lock.read();
          db.search(m, k)
        }
      };

      debug!("Worker {}: Ack: {}", self.id, ack);
      self.tx.send(ack);
    }
  }
}
