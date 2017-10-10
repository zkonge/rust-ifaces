Rust bindings to retrieve network interface information
======================================================================

:Date: 10/10 2017

.. contents::


OS Support
-------------

*   Windows
*   Unix-Like( BSD-Like, XNU, Linux )


Example
-----------

.. code:: toml
    
    [dependencies]
    ifaces = { git = "https://github.com/LuoZijun/rust-ifaces.git" }


.. code:: rust
    
    extern crate ifaces;

    fn main() {
        match ifaces::ifaces() {
            Ok(interfaces) => {
                for interface in interfaces.into_iter() {
                    println!("Found Interface: {:?}", interface)
                }
            },
            Err(_) => println!("Ooops ...")
        };
    }


