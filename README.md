mynumber
================================================================================

[![Build Status](https://travis-ci.org/Tomohiro/mynumber.svg)](https://travis-ci.org/Tomohiro/mynumber)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/Tomohiro/mynumber?svg=true)](https://ci.appveyor.com/project/Tomohiro/mynumber)
[![Coverage Status](https://coveralls.io/repos/Tomohiro/mynumber/badge.svg)](https://coveralls.io/github/Tomohiro/mynumber)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Tomohiro/mynumber/blob/master/LICENSE)

My Number validator for Rust


Installation
--------------------------------------------------------------------------------

Put this in your `Cargo.toml`:

```toml
[dependencies]
mynumber = "0.1"
```


Usage
--------------------------------------------------------------------------------

Put this in your crate root:

```rust
extern crate mynumber;

fn main() {
    match mynumber::verify("123456789018") {
      Ok(()) => println!("valid"),
      Err(e) => println!("invalid: {:?}", e),
    }
}
```

### Verifying an Individual Number a.k.a My Number

```rust
extern crate mynumber;

use mynumber::individual;

fn main() {
    match individual::verify("123456789018") {
      Ok(()) => println!("valid"),
      Err(e) => println!("invalid: {:?}", e),
    }
}
```


### Verifying a Corporate Number

```rust
extern crate mynumber;

use mynumber::corporate;

fn main() {
    match corporate::verify("9234567890123") {
      Ok(()) => println!("valid"),
      Err(e) => println!("invalid: {:?}", e),
    }
}
```


Acknowledgements
--------------------------------------------------------------------------------

- [Ruby - マイナンバーのチェックデジットを計算する - Qiita](http://qiita.com/qube81/items/fa6ef94d3c8615b0ce64)
- [koron/go-mynumber](https://github.com/koron/go-mynumber)


LICENSE
--------------------------------------------------------------------------------

&copy; 2015 Tomohiro TAIRA.

This project is licensed under the MIT license. See [LICENSE](LICENSE) for details.
