use std::io;
use std::os;

fn main() {

  let args = os::args();
  if args.len() != 2 {
    fail!("{} should have one argument", args.get(0));
  }

  let path = Path::new(args.get(1).as_slice());
  let mut file = io::BufferedReader::new(io::File::open(&path));

  // we could add error handling with pattern matching
  for line in file.lines() {
    print!("{}: {}", i, line.unwrap());
  }
}
