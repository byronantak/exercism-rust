pub fn is_armstrong_number(num: u32) -> bool {
    let string_num = num.to_string();
    let power = string_num.len() as u32;
    let mut total_sum_of_powers = 0_u32;
    for chr in string_num.chars() {
        let a = String::from(chr).parse::<u32>().unwrap();
        let b = a.pow(power);
        total_sum_of_powers += b;
    }

    total_sum_of_powers == num
}
