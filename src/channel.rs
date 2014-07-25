mod test;

struct Worker {
  tx: Sender<int>,
  rx: Receiver<int>,
}

impl Worker {
  fn new(tx: Sender<int>, rx: Receiver<int>) -> Worker {
    Worker { tx: tx, rx: rx }
  }
  fn start(&self) {
    loop {
      let msg = self.rx.recv();
      long_computation();
      self.tx.send(msg);
    }
  }
}

fn main() {
  let (worker_tx, io_rx): (Sender<int>, Receiver<int>) = channel();
  let (io_tx, worker_rx): (Sender<int>, Receiver<int>) = channel();

  spawn(proc() {
    let worker: Worker = Worker::new(worker_tx, worker_rx);
    worker.start();
    });

  let tester: test::Tester = test::Tester::new(io_tx, io_rx);
  tester.start();

  0;
}

pub fn long_computation()
{
  std::io::timer::sleep(1000); // Long computation
}
