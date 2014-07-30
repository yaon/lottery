use std::collections::btree::BTree;
use utils::{ Ack, Command, Error, Value, Add, Get, TransactionMeta };

pub struct Loto {
  loto : Option<BTree<String, String>>,
}

impl Loto {
  pub fn new(tree: Option<BTree<String, String>>) -> Loto {
    Loto { loto: tree }
  }

  pub fn command(&mut self, cmd: Command) -> Ack {
    return match cmd {
      Add(m, k, v) => self.add(m, k, v),
      Get(m, k) => self.search(m, k),
    }
  }

  fn search(&mut self, meta: TransactionMeta, key: String) -> Ack {
    return match self.loto {
      Some(ref tree) => {
        match tree.clone().get(key.clone()) {
          Some(value) => { Value(meta, key, value.clone()) },
          None        => { Error(meta, key) }
        }
      }
      None => { Error(meta, key) }
    }
  }

  fn add(&mut self, meta: TransactionMeta, key: String, value: String) -> Ack {
    let new_tree =
      match self.loto {
        Some(ref tree) => Some(tree.clone().insert(key.clone(), value.clone())),
        None           => Some(BTree::new(key.clone(), value.clone(), 2))
      };
    self.loto = new_tree;
    { Value(meta, key, value) }
  }
}
