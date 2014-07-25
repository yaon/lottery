// Max number of keys per node
static ORDER: uint = 4;

enum BPTree<Key, Value>
{
  Node(
    uint,                                          // Branches
    [Key, ..ORDER],                                // Keys
    [Option<Box<BPTree<Key, Value>>>, ..ORDER+1],  // Sons
    Option<Box<BPTree<Key, Value>>>                // Father
  ),
  Leaf(
    Key,
    Value,
    Option<Box<BPTree<Key, Value>>> // Father
  ),
}

fn print<Key, Value>(tree: BPTree<Key, Value>)
{
}


fn compare<Key>(a:&Key, b:&Key) -> int
{
  return -1;
}

fn search<Key, Value>(tree: BPTree<Key, Value>, key: Key) -> int
{
  // loop {
    match tree {
      Node(b, k, s, _) => {
        for i in range(1u, b) {
          if compare(&k[i], &key) < 0 {
          }
        }
        0
      },
      Leaf(_, v, _) => 0
    };
    return 0;
  // }
}

fn insert<Key, Value>(tree: BPTree<Key, Value>, key: Key, value: Value)
{
}

fn delete<Key, Value>(tree: BPTree<Key, Value>, key: Key)
{
}

fn main()
{
  0;
}
