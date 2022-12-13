pub fn read_number_one_or_two_digits(bytes: &mut impl Iterator<Item = u8>) -> u8 {
    let mut num = bytes.next().unwrap() & 0b1111;
    if let Some(other_number) = bytes.next() {
        if other_number != b'\n' {
            num *= 10;
            num += other_number & 0b1111;
            bytes.next();
        }
    }
    num
}

pub fn read_two_digit_number(bytes: &mut impl Iterator<Item = u8>) -> u8 {
    (bytes.next().unwrap() & 0b1111) * 10 + (bytes.next().unwrap() & 0b1111)
}
