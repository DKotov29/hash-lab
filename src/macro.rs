#[macro_export]
macro_rules! prr {
    ( $hex:expr, $origin:expr ) => {
        let h = $hex.as_str();
        let origin: &str = $origin;
        use colored::Colorize;
        println!("{} | {}{}", origin ,&h[..h.len()-4], &h[h.len()-4..].red());
    }
}
