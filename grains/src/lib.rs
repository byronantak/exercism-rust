pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => u64::pow(2_u64, s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    f64::powf(2_f64, 64.0) as u64
}
