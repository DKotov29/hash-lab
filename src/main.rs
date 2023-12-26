extern crate core;

mod add_natural_attack;
mod r#macro;
mod gross;
mod chart_print;
mod lab2;
mod haha2;
mod haha3;

use crypto::digest::Digest;
use crypto::sha2::Sha224;
use distrs::Normal;


fn main() {
    haha3::main();
    return;
    let o = "DavydiukDanylOleksandrovich";

    // let (hash, iteration) = add_natural_attack::att("DavydiukDanylOleksandrovich");

    {
        let mut vec1 = Vec::new();
        let iterations = 100usize;
        for i in 0..=iterations {
            let i = i.to_string();
            let gross = format!("{o}{i}");

            let (hash, iteration) = add_natural_attack::att(gross.as_str());
            vec1.push((i, iteration));
        }
        chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image1_1.svg");
        md(vec1.iter().map(|(_, io2)| *io2));
    }

    {
        // let mut vec1 = Vec::new();
        // let iterations = 100usize;
        // // let iterations = 0usize;
        // for i in 0..=iterations {
        //     let i = i.to_string();
        //     let gross = format!("{o}{i}");
        //
        //     let (hash, iteration) = gross::attack(gross.as_str());
        //     // println!("{iteration} iterations needed");
        //     vec1.push((i, iteration));
        // }
        // chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image_1_2.svg");
        // md(vec1.iter().map(|(_, io2)| *io2));
    }

//Bithday
    let o = "DavydiuknylOleksandrovichFI04";
    {
        // let mut vec1 = Vec::new();
        // let iterations = 100usize;
        // // let iterations = 0usize;
        // for i in 0..=iterations {
        //     let i = i.to_string();
        //     let gross = format!("{o}{i}");
        //
        //     let (hash, iteration) = add_natural_attack::att_bithday(gross.as_str());
        //     // println!("{iteration} iterations needed");
        //     vec1.push((i, iteration));
        // }
        // chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image2_1.svg");
        // md(vec1.iter().map(|(_, io2)| *io2));
    }
    {
        let mut vec1 = Vec::new();
        let iterations = 100usize;
        for i in 0..=iterations {
            let i = i.to_string();
            let gross = format!("{o}{i}");

            let (hash, iteration) = gross::attack_birthday(gross.as_str());
            // println!("{iteration} iterations needed");
            vec1.push((i, iteration));
        }
        chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image_2_2.svg");
        md(vec1.iter().map(|(_, io2)| *io2));
    }
}

fn md(iteer: impl Iterator<Item=usize> + Clone) {
    let m = iteer.clone().sum::<usize>() as f64 / iteer.clone().count() as f64;
    println!("mean: {m}");
    let d = (1f64 / iteer.clone().count() as f64) * (iteer.clone().map(|(io2)| (io2 as f64 - m).powf(2f64)).sum::<f64>());
    println!("Dispersion: {d}");
    let a = 0.05; // significance
    println!("a = {a}, conf = {}", 1f64 - a);
    let z = Normal::ppf(1.0 - a, 0f64, 1f64);

    let z = z * d.sqrt() / iteer.clone().count() as f64;
    println!("[{}, {}]", m - z, m + z );
    println!();
}


