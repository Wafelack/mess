# Blockchain

![CI](![Build & Test](https://github.com/Wafelack/blockchain/workflows/Build%20&%20Test/badge.svg))

A simple blockchain written in Rust

---

```rs
use simple_blockchain::Blockchain;

fn main() {
  let mut chain = Blockchain::<String>::init(8); // Set the type to string and difficulty to 8
  chain.add_block("Hello, World !".to_string());
}
```
