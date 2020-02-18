/// Validates if number string is luhn valid.
/// Returns true if the number string is luhn valid, and false otherwise.
/// The number passed to the function must contain only numeric characters otherwise behavior is undefined.
pub fn valid(number: &str) -> bool {
    let mut checksum = 0;

    let mut iter = number.chars();

    // If the number contains an odd number of digits then 
    // the first digit is an 'odd' digit
    if number.len() % 2 == 1 {
        match iter.next() {
            Some(c) => checksum += checksum_modifier_odd(c),
            None => return true, // Never reached
        }
    }

    // Iterate the rest of the number in pairs starting 
    // with an 'even' number 
    loop {
        match iter.next() {
            Some(c) => checksum += checksum_modifier_even(c),
            None => break,
        }
        match iter.next() {
            Some(c) => checksum += checksum_modifier_odd(c),
            None => break,
        }
    }

    checksum % 10 == 0
}

fn checksum_modifier_odd(c: char) -> u32 {
    numeric_char_to_u32(c)
}

fn checksum_modifier_even(c: char) -> u32 {
    let n = numeric_char_to_u32(c);
    let d = n * 2;
    if d <= 9 {
        return d;
    } else {
        return d - 9;
    }
}

fn numeric_char_to_u32(c: char) -> u32 {
    (c as u32) - ('0' as u32)
}
