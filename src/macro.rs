#[macro_export]
macro_rules! prr {
    ( $hex:expr, $origin:expr ) => {
        let h = $hex.as_str();
        let origin: &str = $origin;
        use colored::Colorize;
        println!("{} | {}{}", origin ,&h[..h.len()-4], &h[h.len()-4..].red());
    }
}

#[macro_export]
macro_rules! prr32 {
    ( $hex:expr, $origin:expr ) => {
        let h = $hex.as_str();
        let origin: &str = $origin;

        println!("{} | {}{}", origin ,&h[..h.len()-8], &h[h.len()-8..].red());
    }
}
