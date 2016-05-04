extern crate ifaces;

#[cfg(not(test))]
fn main () {
    for iface in
        ifaces::Interface::get_all().unwrap()
            .into_iter() {
                println!("{}\t{:?}\t{:?}", iface.name, iface.kind, iface.addr);
            }
}
