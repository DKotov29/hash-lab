use std::collections::HashMap;
use crypto::digest::Digest;
use crypto::sha2::Sha224;

use crate::{prr, prr32};
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
use colored::Colorize;
pub fn att_bithday(s: &str) -> (u32, usize) {
    let mut hasher = Sha224::new();
    hasher.input_str(s);

    let hex = hasher.result_str();
    // prr32!(hex, s);

    let mut map: HashMap<String, usize> = HashMap::new();
    let mut d = 1usize;
    let mut ll = 0u32;
    {
        loop {
            let mut oo = s.to_string();
            oo.push_str(d.to_string().as_str());
            let mut hasher = Sha224::new();
            hasher.input_str(oo.as_str());

            let res = hasher.result_str();
            // if d < 31usize {
                // prr32!(res, oo.as_str());
            // }
            let look = res[res.len() - 8..].to_string();
            ll = u32::from_str_radix(look.as_str(), 16u32).unwrap();
            if let Some(k) = map.get(look.as_str()) {
                // println!();
                let mut hasher = Sha224::new();

                let mut blin = s.to_string();
                blin.push_str((*k).to_string().as_str());
                hasher.input_str(blin.as_str());
                let res1 = hasher.result_str();
                // prr32!(res1, blin.as_str());
                // prr32!(res, oo.as_str());
                break;
            }
            map.insert(look, d);
            d += 1;
        }
    }
    return (ll,d);
}

