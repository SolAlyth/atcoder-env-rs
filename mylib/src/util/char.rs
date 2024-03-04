/// `lower(97..123) - 97 = int(0..26)`  
/// `lower(97..123) ^ 32 = upper_char(65..91)`
pub const lower: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

/// `upper(65..91) - 65 = int(0..26)`  
/// `upper(65..91) ^ 32 = lower(97..123)`
pub const upper: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

pub fn lower_to_us(c: char) -> usize { c as usize - 97 }
pub fn upper_to_us(c: char) -> usize { c as usize - 65 }
