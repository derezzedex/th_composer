pub fn shift(c: char, offset: isize) -> char{
    if !c.is_ascii_alphabetic(){
        return c;
    }

    let number = c.to_digit(36).unwrap() - 10;
    let shifted = ((number as isize + offset) % 26) as u8;
    let caesar = (shifted + 97) as char;

    caesar
}
