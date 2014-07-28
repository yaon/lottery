use utils::{ Ack, Command, Add, Del, Get, ORDER };

pub fn command(cmd: Command) -> Ack {
  return match cmd {
    Add(k, v) => insert(k, v),
    Del(k) => delete(k),
    Get(k) => search(k),
  }
}

enum BPTree {
  Node(
    uint,                               // Branches
    [int, ..ORDER],                     // ints
    [Option<Box<BPTree>>, ..ORDER+1],   // Sons
    Option<Box<BPTree>>                 // Father
  ),
  Leaf(
    u32,                // Hash
    String,             // Key
    String,             // Value
    Option<Box<BPTree>> // Father
  ),
}

pub fn print(tree: BPTree) {
}

fn search(key: String) -> Ack {
  loop {
    // match tree {
    //   Node(b, k, s, _) => {
    //     for i in range(1u, b) {
    //       if &k[i] < &key {
    //       }
    //     }
    //     0i
    //   },
    //   Leaf(_, v, _) => 0i
    // };
    // return 0i;
    break
  }
  return Ack::value(String::from_str("toto"));
}

fn insert(key: String, value: String) -> Ack {
  return Ack::value(String::from_str("toto"));
}

fn delete(key: String) -> Ack {
  return Ack::value(String::from_str("toto"));
}
