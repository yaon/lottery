use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{SOCKET_PATH, Block, Command, Ack, Add, Get, Del, Error, Value};

struct Client {
  client: UnixStream,
  id: uint,
  nbr_request: int,
}

pub struct IThread {
  cmd_chan: Sender<Command>,
  client_chan: Sender<Client>,
  socket: Path,
}

pub struct OThread {
  client_chan: Receiver<Client>,
  ack_chan: Receiver<Ack>,
  clients: Vec<Client>
}

impl IThread {
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
      None => {
        debug!("CMD {}: NONE", unsafe{i});
        None
      },
      Some("add") => {
        debug!("CMD {}: ADD", unsafe{i});
        Some(Add(self.sanitize_str(sliced), self.sanitize_str(sliced)))
      },
      Some("get") => {
        debug!("CMD {}: GET", unsafe{i});
        Some(Get(self.sanitize_str(sliced)))
      },
      err => {
        debug!("CMD {}: OTHER={}", unsafe{i}, err);
        None
      }
    }
  }


}

impl Block for IThread {
  fn new(send: Sender<Command>, recv: Receiver<Ack>) -> IThread {
    let (client_send, _) : (Sender<Client>, Receiver<Client>) = channel(); // FIXME
    let mut ithread = IThread {
      cmd_chan: send,
      client_chan: client_send,
      socket: Path::new(SOCKET_PATH),
    };
    ithread
  }

  fn start(&mut self) -> () {
    self.unlink();

    let stream = match UnixListener::bind(&self.socket) {
      Err(why)   => fail!("failed to bind socket: {}", why),
      Ok(stream) => {println!("Socket bound"); stream},
    };


    for client in stream.listen().incoming() {
      let mut stream = BufferedStream::new(client);
      //&mut self.add_vec(/*client.unwrap()*/);
      let mut nbr_request = 0;
      loop {
        match stream.read_line() {
          Err(e) => {debug!("IOThread: err: {}", e); break},
          Ok(cmd) => match self.parse_cmd(cmd) {
            None => {println!("IOThread: command error. Ignoring")}
            Some(cmd) => {debug!("IOThread: parsed command = [{}]", cmd);
                          self.cmd_chan.send(cmd);}
          }
        }
      }
//      self.update_nbr_request(0, nbr_request);
//
//      for _ in range(0, nbr_request + 1) {
//        let ack = self.recv.recv();
//        self.update_ack(0, ack);
//      }
//
//      self.dump_vec();
    }
  }

  fn exit(&self) -> () {
    self.unlink();
  }
}

impl OThread {

  fn add_vec(&mut self/*, client : UnixStream*/) -> () {
    //let mut client = Ack_Client { /*client: client, */id: self.vec_clients.len(),
    //                              nbr_request: 0, vec_ack: Vec::new() };
    //self.vec_clients.push(client);
  }

  fn update_nbr_request(&mut self, id: uint, nbr_request: int) -> () {
    self.clients.get_mut(id).nbr_request = nbr_request;
  }

  fn update_ack(&mut self, id: uint, ack: Ack) -> () {
    //self.clients.get_mut(id).vec_ack.push(ack)
  }

  fn dump_vec(&self) -> () {
    for i in range(0, self.clients.len()) {
      println!("vec[{}]", i);
      println!("number of requests: {}", self.clients.get(i).nbr_request);
      //for j in range(0, self.vec_clients.get(i).vec_ack.len()) {
      //  let ack = self.vec_clients.get(i).vec_ack.get(j);
      //  match ack {
      //    Error(e) => println!("Error : {}", e),
      //    Value(l,r) => println!("Success: {} {}", l, r),
      //  }
      //}
    }
  }
}

