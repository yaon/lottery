// Max number of keys per node
static ORDER: uint = 4;

enum BPTree
{
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

fn print(tree: BPTree)
{
}

fn search(tree: BPTree, key: String) -> String
{
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
  String::new()
}

fn insert(tree: BPTree, key: String, value: String) -> int
{
  0
}

fn delete(tree: BPTree, key: String) -> int
{
  0
}

fn main()
{
}
