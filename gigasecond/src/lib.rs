use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + 100000000_u64.seconds()
    
    // unimplemented!("What time is a gigasecond later than {}", start);
}
