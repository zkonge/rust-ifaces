# ifaces

## Rust bindings to retrieve network interface information

Now compatible with current Rust 2018 edition.

## OS Support
+ Windows
+ Unix-Like (BSD-Like, XNU, Linux)

## Run
```bash
git clone https://github.com/zkonge/rust-ifaces.git
cargo run --example ifaces
```

## Example

```toml
[dependencies]
ifaces = { git = "https://github.com/zkonge/rust-ifaces.git" }
```
```rust
fn main() {
    match ifaces::ifaces() {
        Ok(interfaces) => {
            for interface in interfaces.iter() {
                println!("Found Interface: {:?}", interface)
            }
        }
        Err(_) => println!("Ooops ...")
    }
}
```

## Thanks

+ dlevy47 <https://github.com/dlevy47/rust-interfaces>, Origin code(linux platform)
+ GGist <https://github.com/GGist/rust-ifaces>, windows platform code
