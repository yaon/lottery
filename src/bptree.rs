use std::collections::btree::BTree;
use utils::{ Ack, Command, Error, Value, Add, Get, TransactionMeta };

pub struct Loto {
  loto : Option<BTree<String, String>>,
}

impl Loto {
  pub fn new(tree: Option<BTree<String, String>>) -> Loto {
    Loto { loto: tree }
  }


  pub fn search(&self, mut meta: TransactionMeta, key: String) -> Ack {
    meta.update_start_op_time();
    return match self.loto {
      Some(ref tree) => {
        match tree.clone().get(key.clone()) {
          Some(value) => { meta.update_end_op_time();
                           Value(meta, key, value.clone()) },
          None        => { meta.update_end_op_time();
                           Error(meta, key) }
        }
      }
      None => { meta.update_end_op_time();
                Error(meta, key) }
    }
  }

  pub fn add(&mut self, mut meta: TransactionMeta, key: String, value: String) -> Ack {
    meta.update_start_op_time();
    let new_tree =
      match self.loto {
        Some(ref tree) => Some(tree.clone().insert(key.clone(), value.clone())),
        None           => Some(BTree::new(key.clone(), value.clone(), 2))
      };
    self.loto = new_tree;
    meta.update_end_op_time();
    { Value(meta, key, value) }
  }
}
