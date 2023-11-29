use crypto::digest::Digest;
use crypto::sha2::Sha224;

use crate::prr;
pub fn att(s: &str) -> (u32, usize) {
    let mut hasher = Sha224::new();
    hasher.input_str(s);

    let hex = hasher.result_str();

    let mut d = 1usize;
    let mut ll = 0u32;
    let looking_for = hex[hex.len() - 4..].to_string();
    {
        loop {
            let mut oo = s.to_string();
            oo.push_str(d.to_string().as_str());
            let mut hasher = Sha224::new();
            hasher.input_str(oo.as_str());
            d += 1;
            let res = hasher.result_str();
            // prr!(res, oo.as_str());
            let look = res[res.len() - 4..].to_string();
            ll = u32::from_str_radix(look.as_str(), 16u32).unwrap();
            if look == looking_for {
                // prr!(res, oo.as_str());
                break;
            }
        }
    }
    return (ll,d);
}