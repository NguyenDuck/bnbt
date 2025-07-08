# NBT

A Rust crate for reading and writing Named Binary Tag (NBT) data, designed for extensibility and performance.

## Examples

Use _nbt_ macro

```rust
use bnbt::nbt;

fn main() {
    let string_tag = nbt!("test", "test");
    let byte_tag = nbt!("test", 1i8);
    let int_tag = nbt!("test", 2);

    let byte_array_tag = nbt!("test", [B, 1, 2, 3]);
    let int_array_tag = nbt!("test", [I, 1, 2, 3]);
    let long_array_tag = nbt!("test", [L, 1, 2, 3]);

    let empty_list_tag = nbt!("test", []);
    let list_of_byte_tag = nbt!("test", [i8; 1, 2]);
    let list_of_compound_tag = nbt!("test", [
        {"byte": 1i8},
        {"int": 2},
    ]);

    let compound_tag = nbt!("test", {});
}
```
