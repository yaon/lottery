pub struct Tester {
  tx: Sender<int>,
  rx: Receiver<int>,
}

impl Tester {
  pub fn new(tx: Sender<int>, rx: Receiver<int>) -> Tester {
    Tester {  tx: tx, rx: rx }
  }
  pub fn start(&self) {
    let mut i = 0;
    loop {
      self.tx.send(i);
      println!("{}", self.rx.recv());
      i += 1
    }
  }
}
