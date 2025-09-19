# A simple key value store / database

As part of some embedded / no_std exploration this project aims to build a simplistic Redis-esque KV store

Goals are mostly learning / improving my understanding of the limitations of the embedded space, getting this to build and run on (mostly emulated) restricted targets

## Usage

```rust
use seidr::Seidr;
use seidr::KeyValueStore;

let mut db = Seidr::default();
db.insert("foo".to_owned(), "bar");
db.insert("bla".to_owned(), "fasel");
let bar = db.remove("foo");
let fasel = db.get("bla");
```

## Naming
[seidr](https://en.wikipedia.org/wiki/Sei%C3%B0r) is an anagram of redis
