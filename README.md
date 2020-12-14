# Blockchain

![CI](https://github.com/wafelack/blockchain/workflows/Build/badge.svg)

A simple blockchain written in Rust

---

```rs
use simple_blockchain::Blockchain;

fn main() {
  let mut chain = Blockchain::<String>::init(8); // Set the type to string and difficulty to 8
  chain.add_block("Hello, World !".to_string());
}
```