use std::io::net::unix::UnixListener;
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::SOCKET_PATH;
use utils::Block;
use utils::Command;
use utils::{Add, Del, Get};

pub struct IOThread {
  send: Sender<Command>,
  recv: Receiver<Command>,
  socket: Path,
}

fn sanitize_str(it: CharSplits<char>) -> String {
  let mut it = it;
  match it.next() {
    None => fail!("Protocol Error: Expected a word but got none."),
    Some(sth) => String::from_str(sth.trim())
  }
}

impl IOThread {
  fn unlink(&self) -> () {
    if self.socket.exists() {
      fs::unlink(&self.socket).unwrap();
    }
  }

  fn parse_cmd(&self, cmd : String) -> Option<Command> {
    static mut i:i32 = 0;
    unsafe{ i += 1 };
    let mut sliced = cmd.as_slice().split(' ');

    match sliced.next() {
      None => {debug!("CMD {}: NONE", unsafe{i}); None},
      Some("ADD") => {debug!("CMD {}: ADD", unsafe{i});
                      Some(Add(sanitize_str(sliced),
                               sanitize_str(sliced)))},
      Some("DEL") => {debug!("CMD {}: DEL", unsafe{i}); Some(Del(sanitize_str(sliced)))},
      Some("GET") => {debug!("CMD {}: GET", unsafe{i}); Some(Get(sanitize_str(sliced)))},
      err => {debug!("CMD {}: OTHER={}", unsafe{i}, err); None}
    }
  }
}


impl Block for IOThread {
  fn new(send: Sender<Command>, recv: Receiver<Command>) -> IOThread {
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

    for client in stream.listen().incoming() {
      let mut stream = BufferedStream::new(client);
      loop {
        match stream.read_line() {
          Err(e) => {debug!("IOThread: err: {}", e); break},
          Ok(cmd) => match self.parse_cmd(cmd) {
            None => {println!("IOThread: command error. Ignoring")}
            Some(cmd) => {debug!("IOThread: parsed command = [{}]", cmd);
                          self.send.send(cmd);}
          }
        }
      }
    }
  }

  fn exit(&self) -> () {
    self.unlink();
    println!("bye IOThread");
  }
}

