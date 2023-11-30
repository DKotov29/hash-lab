use std::collections::HashMap;
use crypto::digest::Digest;
use crypto::sha2::Sha224;
use rand::prelude::*;
use colored::Colorize;
use crate::{prr, prr32};

const ALPH: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}";

//for second attach
pub fn grosserization(s: &str) -> String {
    let mut s1 = s.to_string();
    while s == s1 {
        let mut rng = rand::thread_rng();
        let yoy: usize = rng.gen::<usize>() % s1.len();

        s1.replace_range(yoy..yoy + 1, IteratorRandom::choose_stable(ALPH.chars(), &mut rng).unwrap().to_string().as_str());
    }
    s1
}

pub fn attack(s: &str) -> (u32, usize) {
    let mut hasher = Sha224::new();
    hasher.input_str(s);

    let hex = hasher.result_str();
    // prr!(hex, s);

    let mut d = 1usize;
    let mut ll = 0u32;
    let looking_for = hex[hex.len() - 4..].to_string();
    let mut s = s.to_string();
    {
        loop {
            let oo = grosserization(s.as_str());
            s = oo.clone();

            /// oo.push_str(d.to_string().as_str());
            let mut hasher = Sha224::new();
            hasher.input_str(oo.as_str());
            d += 1;
            let res = hasher.result_str();
            // if d < 31usize {
            //     prr!(res, oo.as_str());
            // }
            let look = res[res.len() - 4..].to_string();
            ll = u32::from_str_radix(look.as_str(), 16u32).unwrap();
            if look == looking_for {
                // prr!(res, oo.as_str());
                break;
            }
        }
    }
    return (ll, d);
}

pub fn attack_birthday(s: &str) -> (u32, usize) {
    let mut hasher = Sha224::new();
    hasher.input_str(s);

    let hex = hasher.result_str();
    // prr32!(hex, s);

    let mut d = 1usize;
    let mut ll = 0u32;
    let mut map: HashMap<String, (usize, String)> = HashMap::new();
    let mut s = s.to_string();
    {
        loop {
            let oo = grosserization(s.as_str());
            s = oo.clone();

            let mut hasher = Sha224::new();
            hasher.input_str(oo.as_str());

            let res = hasher.result_str();
            // if d < 31usize {
            //     prr32!(res, oo.as_str());
            // }
            let look = res[res.len() - 8..].to_string();
            ll = u32::from_str_radix(look.as_str(), 16u32).unwrap();
            if let Some((km, ks)) = map.get(look.as_str()) {
                if ks != &s{
                    // println!();
                    // let mut hasher = Sha224::new();
                    // hasher.input_str(ks.as_str());
                    // let res1 = hasher.result_str();
                    // println!("iteration {km}");
                    // prr32!(res1, ks.as_str());
                    // println!("iteration {d}");
                    // prr32!(res, oo.as_str());
                    break;
                }
            }
            map.insert(look, (d, oo));
            d += 1;
        }
    }
    return (ll, d);
}