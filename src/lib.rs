const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const REVERSE_LOOKUP: [u8; 256] = reverse_lookup();

const fn reverse_lookup() -> [u8; 256] {
    let mut table = [0xff; 256];
    let mut i = 0;
    while i < 64 {
        table[ALPHABET[i] as usize] = i as u8;
        i += 1;
    }
    table
}

pub fn u64_to_str(value: u64) -> String {
    unsafe {
        String::from_utf8_unchecked(Vec::from([
            ALPHABET[((value >> 60) & 0x3F) as usize],
            ALPHABET[((value >> 54) & 0x3F) as usize],
            ALPHABET[((value >> 48) & 0x3F) as usize],
            ALPHABET[((value >> 42) & 0x3F) as usize],
            ALPHABET[((value >> 36) & 0x3F) as usize],
            ALPHABET[((value >> 30) & 0x3F) as usize],
            ALPHABET[((value >> 24) & 0x3F) as usize],
            ALPHABET[((value >> 18) & 0x3F) as usize],
            ALPHABET[((value >> 12) & 0x3F) as usize],
            ALPHABET[((value >> 6) & 0x3F) as usize],
            ALPHABET[(value & 0x3F) as usize],
        ]))
    }
}

pub fn str_to_u64<T>(input: T) -> Option<u64>
where
    T: Into<String>,
{
    let mut result: u64 = 0;

    for (i, c) in input.into().chars().enumerate() {
        if i > 11 {
            return None;
        }
        if c as usize > 256 {
            return None;
        }
        let value = REVERSE_LOOKUP[c as usize];
        result |= (value as u64) << (60 - (i * 6));
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u64() {
        assert_eq!(&u64_to_str(650), "AAAAAAAAAKK");
        assert_eq!(str_to_u64("AAAAAAAAAKK"), Some(650));
        assert_eq!(str_to_u64("AAAAAAAAAKK".to_string()), Some(650));
        assert_eq!(str_to_u64("AAAAAAAAAAA".to_string()), Some(0));
        assert_eq!(&u64_to_str(0), "AAAAAAAAAAA");
        assert_eq!(&u64_to_str(u64::MAX), "P__________");
        assert_eq!(str_to_u64("P__________"), Some(u64::MAX));
        assert!(str_to_u64("🇽s🇰🏳️‍🐱⚧️").is_none());
    }
}
