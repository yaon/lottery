use std::io;
use std::io::net::unix::UnixStream;
use std::os;

use utils::SOCKET_PATH;

mod utils;

fn main() {

  let args = os::args();
  if args.len() != 2 {
    fail!("{} should have one argument", args.get(0));
  }

  let path = Path::new(args.get(1).as_slice());
  let mut file = io::BufferedReader::new(io::File::open(&path));

  let socket = Path::new(SOCKET_PATH);

  let mut stream = match UnixStream::connect(&socket) {
    Err(why) => fail!("Error connecting to server: {}", why),
    Ok(stream) => stream,
  };

  // we could add error handling with pattern matching
  for line in file.lines() {
    match stream.write_str(line.unwrap().as_slice()) {
      Err(why) => fail!("Error sending message to server: {}", why),
      Ok(_) => {},
    }
  }

  stream.close_write();

  println!("{}", stream.read_to_str().unwrap());


}
