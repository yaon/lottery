use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{ SOCKET_PATH, Block, Command, Ack, Error, Value, Add, Get, Del };

struct Ack_Client {
  //client: UnixStream,
  id: uint,
  nbr_request: int,
  vec_ack: Vec<Ack>,
}

pub struct IOThread {
  send: Sender<Command>,
  recv: Receiver<Ack>,
  socket: Path,
  vec_clients: Vec<Ack_Client>,
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


  fn add_vec(&mut self/*, client : UnixStream*/) -> () {
    let mut client = Ack_Client { /*client: client, */id: self.vec_clients.len(),
                                  nbr_request: 0, vec_ack: Vec::new() };
    self.vec_clients.push(client);
  }

  fn update_nbr_request(&mut self, id: uint, nbr_request: int) -> () {
    self.vec_clients.get_mut(id).nbr_request = nbr_request;
  }

  fn update_ack(&mut self, id: uint, ack: Ack) -> () {
    self.vec_clients.get_mut(id).vec_ack.push(ack)
  }

  fn dump_vec(&self) -> () {
    for i in range(0, self.vec_clients.len() - 1) {
      println!("vec[{}]", i);
      println!("number of requests: {}", self.vec_clients.get(i).nbr_request);
      for j in range(0, self.vec_clients.get(i).vec_ack.len()) {
        let ack = self.vec_clients.get(i).vec_ack.get(j);
        match ack {
          &Error(ref e) => println!("Error : {}", e),
          &Value(ref l,ref r) => println!("Success: {} {}", l, r),
        }
      }
    }
  }
}


impl Block for IOThread {
  fn new(send: Sender<Command>, recv: Receiver<Ack>) -> IOThread {
    let mut iothread = IOThread { send: send, recv: recv,
                                  socket: Path::new(SOCKET_PATH),
                                  vec_clients: Vec::new() };
    iothread
  }

  fn start(&mut self) -> () {
    self.unlink();

    let stream = match UnixListener::bind(&self.socket) {
      Err(why)   => fail!("failed to bind socket: {}", why),
      Ok(stream) => {println!("Socket bound"); stream},
    };


    for client in stream.listen().incoming() {
      let mut stream = BufferedStream::new(client);
      &mut self.add_vec(/*client.unwrap()*/);
      let mut nbr_request = 0;
      loop {
        match stream.read_line() {
          Err(e) => {debug!("IOThread: err: {}", e); break},
          Ok(cmd) => match self.parse_cmd(cmd) {
            None => {println!("IOThread: command error. Ignoring")}
            Some(cmd) => {debug!("IOThread: parsed command = [{}]", cmd);
                          nbr_request += 1;
                          self.send.send(cmd);}
          }
        }
      }
      self.update_nbr_request(0, nbr_request);

      for _ in range(0, nbr_request) {
        let ack = self.recv.recv();
        self.update_ack(0, ack);
      }

      self.dump_vec();
    }
  }

  fn exit(&self) -> () {
    self.unlink();
  }
}

