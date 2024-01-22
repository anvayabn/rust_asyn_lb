use std::net::Ipv4Addr; 

use crate::errors::NetworkParseError; 

/* Maximum Length of the IPv4 addr */
pub const MAX_LENGTH: u8 = 32;

/* pub function to return the subnet mask 
    given  length returns a binary mask */

pub fn cidr_mask(len: u8) -> Result<u32, NetworkParseError> {
    /* the length of prefix cannot be greater than the ipaddr */

    if len > MAX_LENGTH { 
        // return 
        return Err(NetworkParseError::NetworkLengthError); 
    }

    let right_len = MAX_LENGTH - len ; 
    let all_bits = u32::MAX as u64; 
    let mask = ( all_bits >> right_len) << right_len ; 

    Ok(mask as u32)    
}

mod test { 
    use super::*;

    #[test]
    fn test_cidr_mask() {
        let len: u8 = 24;
        let expected_mask: u32 = 0xFFFFFF00; // Expected subnet mask for /24
        assert_eq!(cidr_mask(len).unwrap(), expected_mask);
    }
}