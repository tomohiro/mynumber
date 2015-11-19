mynumber
================================================================================

MyNumber validator for Rust



Usage
--------------------------------------------------------------------------------

```rust
extern crate mynumber;

fn main() {
    match mynumber::validate("123456789018") {
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
