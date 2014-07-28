use std::str;
use std::io::net::unix::UnixListener;
use std::io::{fs, Acceptor, Listener, IoError, IoResult};

use utils::SOCKET_PATH;
use utils::Block;
use utils::Command;

pub struct IOThread/*<T>*/ {
  send: Sender<Command>,
  recv: Receiver<Command>,
  socket: Path,
}

fn unoption_str<'a>(s: Option<&'a str>) -> &'a str {
  match s {
    None => "",
    Some(sth) => sth
  }
}

impl IOThread {
  fn unlink(&self) -> () {
    if self.socket.exists() {
      fs::unlink(&self.socket).unwrap();
    }
  }

  fn parse_cmd(&self, cmd : String) -> Option<Command> {
    println!("{}", cmd);
    let mut sliced = cmd.as_slice().split(' ');
    let arg1 = unoption_str(sliced.nth(1));
    let arg2 = unoption_str(sliced.nth(2));

    match sliced.nth(0) {
      None => None,
      Some("ADD") => Some(Command::add(String::from_str(arg1),
                                       String::from_str(arg2))),
      Some("DEL") => Some(Command::del(String::from_str(arg1))),
      Some("GET") => Some(Command::get(String::from_str(arg1))),
      _ => None
    }
  }
}


impl Block for IOThread {
  fn new(send: Sender<Command>, recv: Receiver<Command>) -> IOThread {
    // let listener = TcpListener::bind("0.0.0.0", 3737);
    IOThread {
      send: send,
      recv: recv,
      socket: Path::new(SOCKET_PATH)
    }
  }

  fn start(&self) -> () {
    println!("hello IOThread");

    self.unlink();

    let stream = match UnixListener::bind(&self.socket) {
      Err(why)   => fail!("failed to bind socket: {}", why),
      Ok(stream) => {println!("Socket bound"); stream},
    };

    for mut client in stream.listen().incoming() {
      let cmd = self.parse_cmd(client.read_to_str().unwrap());
      match cmd {
        None => {println!("IOThread: command error. Ignoring")}
        Some(cmd) => {
          println!("IOThread: {}", cmd);
          self.send.send(cmd);
        }
      }
    }
  }

  fn exit(&self) -> () {
    self.unlink();
    println!("bye IOThread");
  }
}

