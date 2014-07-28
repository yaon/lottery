use std::str;
use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, IoError, IoResult};

use utils::{ SOCKET_PATH, Block, Command, Ack };

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

  fn unoption_str<'a>(&self, s: Option<&'a str>) -> &'a str {
    match s {
      None => "",
      Some(sth) => sth
    }
  }

  fn parse_cmd(&self, cmd : String) -> Option<Command> {
    println!("{}", cmd);
    let mut sliced = cmd.as_slice().split(' ');
    let arg1 = self.unoption_str(sliced.nth(1));
    let arg2 = self.unoption_str(sliced.nth(2));

    match sliced.nth(0) { None => None,
      Some("ADD") => Some(Command::add(String::from_str(arg1),
                                       String::from_str(arg2))),
      Some("DEL") => Some(Command::del(String::from_str(arg1))),
      Some("GET") => Some(Command::get(String::from_str(arg1))),
      _ => None
    }
  }


  fn add_vec(self, client : UnixStream) -> () {
    let mut client = Client { client: client, id: self.vec_clients.len(),
                              nbr_request: 0, ack: 0 };
    // self.vec_clients.push(client);
  }

  fn update_nbr_request(self, id: uint, nbr_request: int) -> () {
    // self.vec_clients[id].nbr_request = nbr_request;
  }

  fn update_ack(self, id: uint, ack: int) -> () {
    // self.vec_clients[id].ack = ack;
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
  }
}

