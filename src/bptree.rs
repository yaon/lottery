use std::collections::btree::BTree;
use utils::{ Ack, Command, Error, Value, Add, Del, Get, ORDER };



pub fn command(cmd: Command) -> Ack {
  return match cmd {
    Add(k, v) => insert(k, v),
    Del(k) => delete(k),
    Get(k) => search(k),
  }
}

fn search(key: String) -> Ack {
  let mut loto = BTree::new("k".to_string(), "v".to_string(), 2);
  let get = loto.get(key.clone());
  return match get {
    Some(v) => { Value(key, v) },
    None => { Error(key) }
  }
}

fn insert(key: String, value: String) -> Ack {
  let mut loto = BTree::new("k".to_string(), "v".to_string(), 2);
  loto = loto.insert(key.clone(), value.clone());
  return { Value(key, value) }
}

fn delete(key: String) -> Ack {
  // let loto = BTree::new("k".to_string(), "v".to_string(), 2);
  return { Value(key.clone(), key.clone()) }
}
