fn is_even(a: usize) -> bool {
    a % 2 == 0
}

fn is_valid_luhn_string(code: &str) -> bool {
    let escaped_code = str::replace(code, " ", "")
        .chars()
        .rev()
        .collect::<String>();
    if escaped_code.len() < 2 {
        return false;
    }
    let chars_is_numeric: Vec<bool> = escaped_code.chars().map(|c| c.is_numeric()).collect();
    if chars_is_numeric.contains(&false) {
        return false;
    }
    let mut doubled_even_characters: Vec<char> = Vec::new();
    for (index, chr) in escaped_code.char_indices() {
        if !is_even(index) {
            let char_value = String::from(chr).parse::<u32>().unwrap();
            let mut replaced_value = char_value * 2;
            if replaced_value > 9 {
                replaced_value = replaced_value - 9;
            }
            let new_char_value = replaced_value.to_string();
            doubled_even_characters.push(new_char_value.chars().next().unwrap());
            continue;
        }
        doubled_even_characters.push(chr);
    }
    let mut total = 0_u32;
    for chr in doubled_even_characters {
        let character_value = String::from(chr).parse::<u32>().unwrap();
        total = total + character_value;
    }
    total.rem_euclid(10) == 0
}

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: 'static> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        is_valid_luhn_string(&self.to_string())
    }
}
