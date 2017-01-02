extern crate rustc_serialize as serialize;

use self::serialize::base64::{ToBase64, STANDARD};
use self::serialize::hex::FromHex;

pub fn hex_to_base64_string(str: &str) -> String {
    return str.from_hex().unwrap().to_base64(STANDARD);
}

#[cfg(test)]
mod test {
    use super::hex_to_base64_string;

    #[test]
    fn it_works() {
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", hex_to_base64_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    }
}