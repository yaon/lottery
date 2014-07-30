extern crate time;
use self::time::get_time;

use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{SOCKET_PATH, Command, Ack, Add, Get, Error, Value};
use utils::{TransactionMeta};

pub struct Client {
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

  pub fn new(send: Sender<Command>, client_send: Sender<Client>) -> IThread {
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
            None => {println!("IThread: command error. Ignoring")}
            Some(cmd) => {debug!("IThread: parsed command = [{}]", cmd);
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

  pub fn start(&mut self) {
    loop {
      use std::comm::Select;
      let mut is_client = false;

      {
        let ref client  = self.client_chan;
        let ref ack     = self.ack_chan;
        let s = Select::new();
        let mut handle1 = s.handle(client);
        let mut handle2 = s.handle(ack);
        unsafe {
          handle1.add();
          handle2.add();
        }
        is_client = (s.wait() == handle1.id())
      }

      if is_client {
        let cli = self.client_chan.recv();
        self.add_client(cli)
      } else {
        let ack = self.ack_chan.recv();
        self.dispatch_ack(ack)
      }
    }
  }

  pub fn add_client(&mut self, client : Client) {
    for i in range(0, self.clients.len) {
      if self.clients.get(i).id = i {
        println!("OThread: found same client id({}) in clients vec", i);
        break;
      }
    }
    self.clients.push(client);
  }

  pub fn dispatch_ack(&mut self, ack : Ack) {
  }
}

