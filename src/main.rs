// Max number of keys per node
static ORDER: uint = 4;

enum BPTree<Key, Value>
{
  Node(
    uint,                                     // Branches
    [Key, ..ORDER],                           // Keys
    [Box<Option<BPTree<Key, Value>>>, ..ORDER+1],  // Sons
    Box<Option<BPTree<Key, Value>>>                // Father
  ),
  Leaf(
    Key,
    Value,
    Box<Option<BPTree<Key, Value>>> // Father
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
          if compare(&k[1], &key) < 0 {
          }
        }
        0i
      },
      Leaf(_, v, _) => 0i
    };
    return 0i;
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
  0i;
}
