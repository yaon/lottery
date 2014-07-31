extern crate time;
use self::time::get_time;

use std::io::net::unix::{UnixListener, UnixStream};
use std::io::{fs, Acceptor, Listener, BufferedStream};
use std::str::CharSplits;

use utils::{SOCKET_PATH, Command, Ack, Add, Get, Error, Value};
use utils::{TransactionMeta};

pub struct Client {
  client:       UnixStream,
  id:           uint,
  nb_req:       uint,
  nb_ack:       uint,
  wait_end:     bool,
}

pub enum IOCmd {
  New(uint, UnixStream), // id client, socket
  Cmd(uint),             // id_client
  End(uint)              // id_client
}

pub struct IThread {
  cmd_chan:     Sender<Command>,
  client_chan:  Sender<IOCmd>,
  socket:       Path,
}

pub struct OThread {
  client_chan:  Receiver<IOCmd>,
  ack_chan:     Receiver<Ack>,
  clients:      Vec<Client>,
  acks:         Vec<Box<Ack>>,
}

impl Drop for Client {
  fn drop(&mut self) {
    drop(&self.client);
  }
}

impl Clone for Client {
  fn clone(&self) -> Client {
    Client {id: self.id, nb_req: self.nb_req, nb_ack: self.nb_ack,
            wait_end: self.wait_end, client: self.client.clone()}
  }
}

  pub fn send_ack(ack: Ack, mut clt: Client) {
    match ack {
      Error(m, s) => {
        clt.client.write_str((format!("Error: {}\n{}",
                                      s, dump_meta(m))).as_slice());
        ()
      }
      Value(m, k, v) => {
        clt.client.write_str((format!("Success: {} => {}\n{}",
                                      k, v, dump_meta(m)).as_slice()));
        ()
      }
    }
  }

  pub fn dump_meta(meta: TransactionMeta) -> String {
    return (format!("id_client : {}\n
            id_transaction: {}\n
            open_time: {}\n
            close_time: {}\n
            start_query_time: {}\n
            end_query_time: {}",
            meta.id_client, meta.id_transaction,
            meta.open_time, meta.close_time,
            meta.start_op_time, meta.end_op_time)).as_slice().to_string()
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

  fn parse_cmd(&self, client_id: uint, cmd : String) -> Option<Command> {
    static mut i:uint = 0;
    unsafe{ i += 1 };
    let mut sliced = cmd.as_slice().split(' ');
    let trans = unsafe{i};
    let meta = TransactionMeta::new(client_id, trans, get_time());
    match sliced.next() {
      None => {
        None
      },
      Some("add") | Some("ADD") => {
        // ugly fix, i don't know how it's working but hey
        let mut sl = sliced;
        let mut sl2 = String::from_str(sl.next().unwrap());
        debug!("CMD {}: ADD", trans);
        Some(Add(meta, sl2, self.sanitize_str(sl)))
      },
      Some("get") | Some("GET") => {
        Some(Get(meta, self.sanitize_str(sliced)))
      },
      err => {
        None
      }
    }
  }

  pub fn new(send: Sender<Command>, client_send: Sender<IOCmd>) -> IThread {
    let ithread = IThread {
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
      Ok(stream) => {println!("IThread: Socket bound"); stream},
    };

    static mut i :uint = 0;
    let mut client_id = unsafe{i};

    for client in stream.listen().incoming() {
      let mut stream = BufferedStream::new(client.clone());

      {
        debug!("$$ Sending NEW client {}", client_id);
        self.client_chan.send(New(client_id, client.clone().unwrap()))
      }

      loop {
        match stream.read_line() {
          Ok(cmd) => match self.parse_cmd(client_id, cmd) {
            None => {println!("IThread: command error. Ignoring")}
            Some(cmd) => {debug!("IThread: parsed command = [{}]", cmd);
              self.cmd_chan.send(cmd);
              debug!("$$ Sending CMD to client {}", client_id);
              self.client_chan.send(Cmd(client_id))
            }
          },
          // le compilo dit que y'a que EndOfFile donc pas d'erreurs
          Err(_) => break
        }
      }

      debug!("$$ Sending END client {}", client_id);
      self.client_chan.send(End(client_id));

      unsafe { i += 1 };
      client_id = unsafe{i};
    }
  }

  fn exit(&self) -> () {
    self.unlink();
  }
}

impl OThread {

  pub fn new(client:Receiver<IOCmd>, ack:Receiver<Ack>) -> OThread {
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
      let mut is_client;

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
        is_client = s.wait() == handle1.id()
      }

      if is_client {
        let cli = self.client_chan.recv();
        debug!("** RECEIVED CLIENT !");
        self.handle_client(cli)
      } else {
        let ack = self.ack_chan.recv();
        debug!("** Received ack ! {}", ack);
        self.dispatch_ack(ack)
      }
    }
  }

  fn handle_client(&mut self, cmd: IOCmd) {
    match cmd {
      New(cli_id, stream) => self.add_client(cli_id, stream),
      Cmd(cli_id) => self.incr_client_cmd(cli_id),
      End(cli_id) => self.try_drop_client(cli_id, true)
    }
  }

  fn incr_client_cmd(&mut self, id:uint) {
    let (_, client) = self.find_client(id);
    client.nb_req += 1
  }

  fn try_drop_client(&mut self, id: uint, get_end_cmd: bool) {
    let (_, client) = self.find_client(id);
    if get_end_cmd {
      client.wait_end = true;
    }

    if !client.wait_end { return }

    if client.nb_req == client.nb_ack {
      client.client.close_read();
      client.client.close_write();
      drop(client)
    }
  }

  pub fn add_client(&mut self, client: uint, stream: UnixStream) {
    let cli = Client {id: client, client: stream,
                      nb_req: 0, nb_ack: 0, wait_end: false};
    self.clients.push(cli)
  }

  fn find_client<'a>(&'a mut self, id: uint) -> (uint, &'a mut Client) {
    let idx = match self.clients.mut_iter().position({|e| e.id == id}) {
      None => {
        debug!("Client {} not found, waiting the client...", id);
        ::std::io::timer::sleep(50);
        let (idx, _) = self.find_client(id);
        idx
      }
      Some(c) => c
    };

    (idx, self.clients.get_mut(idx))
  }

  pub fn dispatch_ack(&mut self, ack : Ack) {
    debug!("Dispatching ACK");
    let meta = ack.meta();
    let (idx, _) = {
      let (idx, client) : (uint, &mut Client) = self.find_client(meta.id_client);
      client.nb_ack += 1;
      send_ack(ack.clone(), client.clone());
      (idx, client.clone())
    };
    self.acks.push(box ack.clone());

    debug!("Before Try-Drop");
    self.try_drop_client(idx, false);
    debug!("End Dispatching ACK");
  }
}

