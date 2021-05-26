fn main() {
    match ifaces::ifaces() {
        Ok(interfaces) => {
            for interface in interfaces.iter() {
                println!("Found Interface: {:?}", interface)
            }
        }
        Err(_) => println!("Ooops ..."),
    };
}
