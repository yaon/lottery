use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{ SOCKET_PATH, Block, Command, Ack, Add, Get, Del };

struct Client {
  client: UnixStream,
  id: uint,
  nbr_request: int,
  ack: int,
}

pub struct IOThread {
  send: Sender<Command>,
  recv: Receiver<Ack>,
  socket: Path,
  vec_clients: Vec<Client>,
}

impl IOThread {
  fn unlink(&self) -> () {
    if self.socket.exists() {
      fs::unlink(&self.socket).unwrap();
    }
  }

  fn sanitize_str(&self, it: CharSplits<char>) -> String {
    let mut it = it;
    match it.next() {
      None => fail!("Protocol Error: Expected a word but got none."),
      Some(sth) => String::from_str(sth.trim())
    }
  }

  fn parse_cmd(&self, cmd : String) -> Option<Command> {
    static mut i:i32 = 0;
    unsafe{ i += 1 };
    let mut sliced = cmd.as_slice().split(' ');
    match sliced.next() {
      None => {debug!("CMD {}: NONE", unsafe{i}); None},
      Some("ADD") => {debug!("CMD {}: ADD", unsafe{i});
                      Some(Add(self.sanitize_str(sliced),
                               self.sanitize_str(sliced)))},
      Some("DEL") => {debug!("CMD {}: DEL", unsafe{i}); Some(Del(self.sanitize_str(sliced)))},
      Some("GET") => {debug!("CMD {}: GET", unsafe{i}); Some(Get(self.sanitize_str(sliced)))},
      err => {debug!("CMD {}: OTHER={}", unsafe{i}, err); None}
    }
  }


  fn add_vec(&mut self, client : UnixStream) -> () {
    let mut client = Client { client: client, id: self.vec_clients.len(),
                              nbr_request: 0, ack: 0 };
    self.vec_clients.push(client);
  }

  fn update_nbr_request(&mut self, id: uint, nbr_request: int) -> () {
    self.vec_clients.get_mut(id).nbr_request = nbr_request;
  }

  fn update_ack(&mut self, id: uint, ack: int) -> () {
    self.vec_clients.get_mut(id).ack = ack;
  }
}


impl Block for IOThread {
  fn new(send: Sender<Command>, recv: Receiver<Ack>) -> IOThread {
    let mut iothread = IOThread { send: send, recv: recv,
                                  socket: Path::new(SOCKET_PATH),
                                  vec_clients: Vec::new() };
    iothread
  }

  fn start(&self) -> () {
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
  }
}

