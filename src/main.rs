use utils::{Block, Command};
use io_thread::IOThread;
use pp_thread::TPP;

mod utils;
mod io_thread;
mod pp_thread;

fn main() {
  let (s1, r1): (Sender<Command>, Receiver<Command>) = channel();
  let (s2, r2): (Sender<Command>, Receiver<Command>) = channel();

  spawn(proc() {
    let tio: IOThread = Block::new(s2, r1);
    tio.start();
  });

  spawn(proc() {
    let tpp: TPP = Block::new(s1, r2);
    tpp.start();
  });


  loop {
    std::io::timer::sleep(1000);
  }
}
