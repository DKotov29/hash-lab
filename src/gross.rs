use crypto::digest::Digest;
use crypto::sha2::Sha224;
use rand::prelude::*;

const ALPH: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}";

//for second attach
pub fn grosserization(s: &str) -> String {
    let mut rng = rand::thread_rng();
    let yoy: usize = rng.gen::<usize>() % s.len();
    let mut s = s.to_string();
    s.replace_range(yoy..yoy + 1, IteratorRandom::choose_stable(ALPH.chars(), &mut rng).unwrap().to_string().as_str());

    s
}

pub fn attack(s: &str) -> (u32, usize) {
    let mut hasher = Sha224::new();
    hasher.input_str(s);

    let hex = hasher.result_str();

    let mut d = 1usize;
    let mut ll = 0u32;
    let looking_for = hex[hex.len() - 4..].to_string();
    let mut s = s.to_string();
    {
        loop {
            let mut oo = grosserization(s.as_str());
            s = oo.clone();
            /// oo.push_str(d.to_string().as_str());
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
    return (ll, d);
}