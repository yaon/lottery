use block::Block;
use io_thread::IOThread;
use pp_thread::TPP;

mod block;
mod io_thread;
mod pp_thread;

fn main() {
  let (s1, r1): (Sender<uint>, Receiver<uint>) = channel();
  let (s2, r2): (Sender<uint>, Receiver<uint>) = channel();

  spawn(proc() {
    let tio: IOThread = Block::new(s2, r1);
    tio.start();
  });

  spawn(proc() {
    let tpp: TPP = Block::new(s1, r2);
    tpp.start();
  });
}
