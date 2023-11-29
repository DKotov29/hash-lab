mod add_natural_attack;
mod r#macro;
mod gross;
mod chart_print;

use crypto::digest::Digest;


fn main() {
    let o = "DavydiukDanylOleksandrovich";

    // let (hash, iteration) = add_natural_attack::att("DavydiukDanylOleksandrovich");

    {
        // let mut vec1 = Vec::new();
        // let iterations = 100usize;
        // for i in 0..=iterations {
        //     let i = i.to_string();
        //     let gross = format!("{o}{i}");
        //
        //     let (hash, iteration) = add_natural_attack::att(gross.as_str());
        //     vec1.push((i, iteration));
        // }
        // chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image1_1.svg");
        // let m = vec1.iter().map(|(_, io2)| *io2).sum::<usize>() as f64 / vec1.len() as f64;
        // println!("mean: {m}");
        // let d = (1f64 / vec1.len() as f64) * (vec1.iter().map(|(_, io2)| (*io2 as f64 - m).powf(2f64)).sum::<f64>());
        // println!("Dispersion: {d}");
    }
    {
        let mut vec1 = Vec::new();
        let iterations = 100usize;
        for i in 0..=iterations {
            let i = i.to_string();
            let gross = format!("{o}{i}");

            let (hash, iteration) = gross::attack(gross.as_str());
            vec1.push((i, iteration));
        }
        chart_print::chhhart(&vec1.iter().map(|(s, u)| (s.clone(), *u as f32)).collect(), "image_1_2.svg");
        let m = vec1.iter().map(|(_, io2)| *io2).sum::<usize>() as f64 / vec1.len() as f64;
        println!("mean: {m}");
        let d = (1f64 / vec1.len() as f64) * (vec1.iter().map(|(_, io2)| (*io2 as f64 - m).powf(2f64)).sum::<f64>());
        println!("Dispersion: {d}");
    }
}


