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

#[inline(always)]
pub fn from_u64(value: u64) -> [u8; 11] {
    [
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
    ]
}

#[inline(always)]
pub fn to_u64(input: &[u8; 11]) -> u64 {
    let mut result: u64 = 0;

    for (i, &c) in input.iter().enumerate() {
        let value = REVERSE_LOOKUP[c as usize];
        result |= (value as u64) << (60 - (i * 6));
    }

    result
}

#[inline(always)]
pub fn string_to_u64(input: String) -> Option<u64> {
    let mut result: u64 = 0;

    for (i, c) in input.chars().enumerate() {
        if c as usize > 256 {
            return None;
        }
        let value = REVERSE_LOOKUP[c as usize];
        result |= (value as u64) << (60 - (i * 6));
    }

    Some(result)
}

#[inline(always)]
pub fn to_str(input: &[u8; 11]) -> &str {
    unsafe { std::str::from_utf8_unchecked(input) }
}

#[inline(always)]
pub fn b64_to_string(input: &[u8; 11]) -> String {
    // Safe to use unsafe here as we know the input array contains valid ASCII
    // characters from our ALPHABET array
    unsafe { String::from_utf8_unchecked(input.to_vec()) }
}

// Alternative safe version
pub fn b64_to_string_safe(input: &[u8; 11]) -> String {
    String::from_utf8_lossy(input).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        let value: u64 = 650;
        let encoded = from_u64(value);
        assert_eq!(encoded, *b"AAAAAAAAAKK");
    }

    #[test]
    fn test_to_u64() {
        let encoded: [u8; 11] = *b"AAAAAAAAAKK";
        let decoded = to_u64(&encoded);
        assert_eq!(decoded, 650);
    }

    #[test]
    fn test_string_to_u64() {
        let encoded: String = "AAAAAAAAAKK".to_string();
        let decoded = string_to_u64(encoded);
        assert_eq!(decoded, Some(650));
    }

    #[test]
    fn test_to_str() {
        let input: [u8; 11] = *b"AAAAAAAAAKK";
        let result = to_str(&input);
        assert_eq!(result, "AAAAAAAAAKK");
    }

    #[test]
    fn test_b64_to_string() {
        let input: [u8; 11] = *b"AAAAAAAAAKK";
        let result = b64_to_string(&input);
        assert_eq!(result, "AAAAAAAAAKK");
    }

    #[test]
    fn test_b64_to_string_safe() {
        let input: [u8; 11] = *b"AAAAAAAAAKK";
        let result = b64_to_string_safe(&input);
        assert_eq!(result, "AAAAAAAAAKK");
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero value
        let zero_value: u64 = 0;
        let zero_encoded = from_u64(zero_value);
        assert_eq!(zero_encoded, *b"AAAAAAAAAAA");
        let zero_decoded = to_u64(&zero_encoded);
        assert_eq!(zero_decoded, zero_value);

        // Test with maximum value
        let max_value: u64 = u64::MAX;
        let max_encoded = from_u64(max_value);
        assert_eq!(max_encoded, *b"P__________");
        let max_decoded = to_u64(&max_encoded);
        assert_eq!(max_decoded, max_value);
    }

    #[test]
    fn test_invalid_cases() {
        assert!(string_to_u64("🇽s🇰🏳️‍🐱⚧️".to_string()).is_none());
    }
}
