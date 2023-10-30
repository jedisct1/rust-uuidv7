UUIDv7 for Rust

A simple [UUIDv7](https://uuid6.github.io/uuid6-ietf-draft/) crate.

```rust
let uid = uuidv7::create();
```

This returns a string.

Alternatively, a 16-byte binary array can be returned:

```rust
let raw_uid = uuidv7::create_raw();
```
