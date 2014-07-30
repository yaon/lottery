extern crate time;
use self::time::get_time;

use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{SOCKET_PATH, Command, Ack, Add, Get, Error, Value};
use utils::{TransactionMeta};

struct Client {
  client:       UnixStream,
  id:           u32,
  nbr_request:  int,
}

pub struct IThread {
  cmd_chan:     Sender<Command>,
  client_chan:  Sender<Client>,
  socket:       Path,
}

pub struct OThread {
  client_chan:  Receiver<Client>,
  ack_chan:     Receiver<Ack>,
  clients:      Vec<Client>,
  acks:         Vec<Ack>,
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

  fn parse_cmd(&self, client_id: u32, cmd : String) -> Option<Command> {
    static mut i:u32 = 0;
    unsafe{ i += 1 };
    let mut sliced = cmd.as_slice().split(' ');
    let trans = unsafe{i};
    let meta = TransactionMeta::new(client_id, trans, get_time());
    match sliced.next() {
      None => {
        debug!("CMD {}: NONE", trans);
        None
      },
      Some("add") => {
        debug!("CMD {}: ADD", trans);
        Some(Add(meta, self.sanitize_str(sliced), self.sanitize_str(sliced)))
      },
      Some("get") => {
        debug!("CMD {}: GET", trans);
        Some(Get(meta, self.sanitize_str(sliced)))
      },
      err => {
        debug!("CMD {}: OTHER={}", trans, err);
        None
      }
    }
  }

  pub fn new(send: Sender<Command>, recv: Receiver<Ack>) -> IThread {
    let (client_send, _) : (Sender<Client>, Receiver<Client>) = channel(); // FIXME
    let mut ithread = IThread {
      cmd_chan: send,
      client_chan: client_send,
      socket: Path::new(SOCKET_PATH),
    };
    ithread
  }

  pub fn start(&mut self) -> () {
    self.unlink();

    let stream = match UnixListener::bind(&self.socket) {
      Err(why)   => fail!("failed to bind socket: {}", why),
      Ok(stream) => {println!("Socket bound"); stream},
    };


    for client in stream.listen().incoming() {
      static mut i :u32 = 0;
      unsafe { i += 1 };
      let client_id = unsafe{i};
      let mut stream = BufferedStream::new(client.clone());
      let mut nbr_request = 0;
      loop {
        match stream.read_line() {
          Ok(cmd) => match self.parse_cmd(client_id, cmd) {
            None => {println!("IOThread: command error. Ignoring")}
            Some(cmd) => {debug!("IOThread: parsed command = [{}]", cmd);
                          nbr_request += 1;
                          self.cmd_chan.send(cmd);}
          },
          // le compilo dit que y'a que EndOfFile donc pas d'erreurs
          Err(_) => break
        }
        self.client_chan.send(Client {
          client: client.clone().unwrap(),
          id:     client_id,
          nbr_request: nbr_request
        });
      }
    }
  }

  fn exit(&self) -> () {
    self.unlink();
  }
}

impl OThread {

  pub fn new(client:Receiver<Client>, ack:Receiver<Ack>) -> OThread {
    OThread {
      client_chan:  client,
      ack_chan:     ack,
      clients:      Vec::new(),
      acks:         Vec::new(),
    }
  }

  pub fn start(&self) {
    let client  = self.client_chan;
    let ack     = self.ack_chan;
    loop {select!(
      c = client.recv()   => self.add_client(c),
      a = ack.recv()      => self.dispatch_ack(a)
    )}
  }

  pub fn add_client(&self, client : Client) {
    
  }

  pub fn dispatch_ack(&self, ack : Ack) {
  
  }

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

