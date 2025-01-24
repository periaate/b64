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
