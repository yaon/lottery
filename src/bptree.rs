use std::collections::btree::BTree;
use utils::{ Ack, Command, Error, Value, Add, Del, Get };

pub struct Loto {
  loto : Option<BTree<String, String>>,
}

impl Loto {
  pub fn new(tree: Option<BTree<String, String>>) -> Loto {
    Loto { loto: tree }
  }

  pub fn command(&mut self, cmd: Command) -> Ack {
    return match cmd {
      Add(k, v) => self.add(k, v),
      Del(k) => self.search(k),
      Get(k) => self.search(k),
    }
  }

  fn search(&self, key: String) -> Ack {
    return match self.loto {
      Some(ref tree) => {
        match tree.clone().get(key.clone()) {
          Some(value) => { Value(key, value.clone()) },
          None        => { Error(key) }
        }
      }
      None => { Error(key) }
    }
  }

  fn add(&mut self, key: String, value: String) -> Ack {
    let new_tree =
      match self.loto {
        Some(ref tree) => Some(tree.clone().insert(key.clone(), value.clone())),
        None           => Some(BTree::new(key.clone(), value.clone(), 2))
      };
    self.loto = new_tree;
    { Value(key, value) }
  }
}
