use std::collections::HashMap;
use std::convert::TryInto;
use crypto::digest::Digest;
use rand::Rng;

use crypto::sha2::Sha224;
use num_bigfloat::BigFloat;
use num_bigint::BigUint;

struct Tools;

impl Tools {
    fn hash(text: &str) -> String {
        let mut hasher = Sha224::new();
        hasher.input_str(text.trim_start_matches("0"));
        hasher.result_str()[..36].to_string()
    }

    fn get_random_bytes(bits: i32) -> String {
        let mut s = String::with_capacity(bits as usize);
        for _ in 0..bits {
            let i = rand::thread_rng().gen_range((0..2));
            s.push(i.to_string().chars().next().unwrap());
        }
        BigUint::from_str_radix(s.as_str(), 2u32).unwrap().to_str_radix(16u32)
    }

    fn get_redundancy_table(
        k: i64,
        l: i64,
        trun: i32,
    ) -> HashMap<String, String> {
        let mut redundancy_table = HashMap::new();
        let l = 2_i64.pow(l.try_into().unwrap());
        let k = 2_i64.pow(k.try_into().unwrap());
        for i in 0..k {
            let x0 = Tools::get_random_bytes(trun);
            let mut x_prev = x0.clone();
            for _ in 0..l {
                x_prev = Tools::hash(R(x_prev.as_str()).as_str()).chars().collect();
            }
            redundancy_table.insert(x0, x_prev);
        }
        redundancy_table
    }
}

use num_traits::{Num, One, Zero};
use num_traits::real::Real;

fn R(x: &str) -> String {
    let mut s = String::with_capacity(128 - truncation as usize);
    for _ in 0..(128 - truncation) {
        let i = rand::thread_rng().gen_range((0..2));
        s.push(i.to_string().chars().next().unwrap());
    }
    format!("{}{x}", BigUint::from_str_radix(s.as_str(), 2u32).unwrap().to_str_radix(16u32))
}
fn R_r() -> String {
    let mut s = String::with_capacity(128 - truncation as usize);
    for _ in 0..(128 - truncation) {
        let i = rand::thread_rng().gen_range((0..2));
        s.push(i.to_string().chars().next().unwrap());
    }
    s
}


use rayon::prelude::*;

fn predict_probability_by_hellmann_theorem(k: i64, l: i64, trunc: i32) -> f64 {
    let n = 2_i64.pow(trunc as u32);
    let mut sum = 0.0;

    let o_blya = (1..=k).into_par_iter().map(|i| {
        let l = BigFloat::from(l);
        let i = BigFloat::from(i);
        let n = BigFloat::from(n);

        if n == BigFloat::zero() || l == -(i * n) {
            BigFloat::zero()
        } else {
            let b = (-i * l * n *((i * l)/n).powf(l + BigFloat::one()) + i * l * (l + BigFloat::from(2))* n - (l + BigFloat::one()) * n*n)/(n * (i * l - n));
            //println!("{b}");
            b
        }
    }).sum::<BigFloat>();
    // println!("сума {o_blya}");
    sum = o_blya.to_f64();
    sum /= n as f64;
    sum
}

fn first_attack() {
    let k_exponents = vec![10, 12, 14];
    let l_exponents = vec![5, 6, 7];
    for &k in &k_exponents {
        for &l in &l_exponents {
            let mut success = 0;
            let redundancy_table =
                Tools::get_redundancy_table(k, l, truncation);
            // println!("{redundancy_table:?}");
            let l = 2_i64.pow(l.try_into().unwrap());
            for _ in 0..N {
                let povid = Tools::get_random_bytes(256);
                // println!("povidomlennya {povid}");
                let mut random_hash = Tools::hash(&povid)
                    .chars()
                    .collect::<String>()
                    .to_lowercase();
                // println!("yogo hash {random_hash}");
                for _ in 0..l {
                    if redundancy_table.values().any(|value| value == &random_hash) {
                        success += 1;
                        break;
                    } else {
                        random_hash = Tools::hash(R(random_hash.as_str()).as_str())
                            .chars()
                            .collect::<String>()
                            .to_lowercase();
                    }
                }
            }
            let k_for_prediction = 2_i64.pow(k.try_into().unwrap());
            let l_for_prediction = 2_i64.pow(l.try_into().unwrap());
            println!("------------------------------------------------------------------");
            println!("K: 2^{}", k);
            println!("L: 2^{}", l);
            println!(
                "Віщування Хелмана: {}",
                predict_probability_by_hellmann_theorem(
                    k_for_prediction,
                    l_for_prediction,
                    truncation,
                )
            );
            println!("Коефіцієнт успішності: {}", success as f64 / N as f64);
            println!();
        }
    }
}

fn second_attack() {
    let k_exponents = vec![5, 6, 7];
    let l_exponents = vec![5, 6, 7];
    for &k in &k_exponents {
        for &l in &l_exponents {
            let mut success = 0;
            let mut redundancy_tables = HashMap::new();
            let k = 2_i64.pow(k.try_into().unwrap());
            let l = 2_i64.pow(l.try_into().unwrap());
            for i in 0..k {
                let function = R_r();
                let table = Tools::get_redundancy_table(k, l, truncation);
                redundancy_tables.insert(function, table);
            }
            for _ in 0..N {
                let mut hashes = vec![];
                let mut random_hash = Tools::hash(&Tools::get_random_bytes(256))
                    .chars()
                    .collect::<String>()
                    .to_lowercase();
                for _ in 0..l {
                    hashes.push(random_hash.clone());
                }
                let mut out = false;
                for j in 0..l {
                    for (function, table) in &redundancy_tables {
                        if table.values().any(|value| value == hashes.get(j as usize).unwrap()) {
                            success += 1;
                            out = true;
                            break;
                        } else {
                            let mut hahahahha = function.clone();
                            hahahahha.push_str(&hashes[j as usize]);
                            hashes[j as usize] = Tools::hash(&hahahahha)
                                .chars()
                                .collect::<String>()
                                .to_lowercase();
                        }
                    }
                    if out {
                        break;
                    }
                }
            }
            let k_for_prediction = 2_i64.pow(k.try_into().unwrap());
            let l_for_prediction = 2_i64.pow(l.try_into().unwrap());
            println!("------------------------------------------------------------------");
            println!("K: 2^{}", k);
            println!("L: 2^{}", l);
            println!(
                "Віщування Хелмана: {}",
                predict_probability_by_hellmann_theorem(
                    k_for_prediction.try_into().unwrap(),
                    l_for_prediction.try_into().unwrap(),
                    truncation,
                )
            );
            println!("Коефіцієнт успішності: {}", success as f64 / N as f64);
            println!();
        }
    }
}

const N: i32 = 10000;
const truncation: i32 = 16;

pub fn main() {
    second_attack();
}


