# Rust bindings to retrieve network interface information

This library contains functionality to retrieve network interface information on Linux machines.

## Example usage

See `examples/ifaces.rs` for an example of printing out network interfaces on a machine:

```rust
extern crate ifaces;

fn main () {
    for iface in
        ifaces::Interface::get_all().unwrap()
            .into_iter() {
                println!("{}\t{:?}\t{:?}", iface.name, iface.kind, iface.addr);
            }
}
```

On my machine, this prints out:

```
$ cargo run --example ifaces
lo0	Ipv6	Some(V6([::1]:0))
lo0	Ipv4	Some(V4(127.0.0.1:0))
lo0	Ipv6	Some(V6([fe80::1]:0))
en5	Ipv4	Some(V4(192.168.168.133:0))
```
