use std::net::Ipv4Addr; 

/* Here we define a ipv4 cidr structure */

/* using the derive attribute so that it can be cloned, copied 
    hashed and equated   */
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Ipv4Cidr { 
    /* eg 127.0.0.1/24 
        here the addr is 127.0.0.1
        and the len represents the prefix length*/
    addr : Ipv4Addr,
    len: u8,
}




// /* Some implementations of Ipv4Cidr  */
// impl Ipv4Addr {

// }
