// AUTOGENERATED FROM index-windows-1253.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-windows-1253.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: 49fdc881a3488904dd1e8dfba9aef3258454249958b611bcded1d4c981ab5561
// Date: 2016-01-20

#[allow(dead_code)] const X: u16 = 0xffff;

const FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 136, 8240, 138, 8249, 140,
    141, 142, 143, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 152, 8482,
    154, 8250, 156, 157, 158, 159, 160, 901, 902, 163, 164, 165, 166, 167, 168,
    169, X, 171, 172, 173, 174, 8213, 176, 177, 178, 179, 900, 181, 182, 183,
    904, 905, 906, 187, 908, 189, 910, 911, 912, 913, 914, 915, 916, 917, 918,
    919, 920, 921, 922, 923, 924, 925, 926, 927, 928, 929, X, 931, 932, 933,
    934, 935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945, 946, 947, 948,
    949, 950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960, 961, 962, 963,
    964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, X,
]; // 128 entries

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 129, 0, 0, 0, 0, 0, 0, 136, 0,
    138, 0, 140, 141, 142, 143, 144, 0, 0, 0, 0, 0, 0, 0, 152, 0, 154, 0, 156,
    157, 158, 159, 160, 0, 0, 163, 164, 165, 166, 167, 168, 169, 0, 171, 172,
    173, 174, 0, 176, 177, 178, 179, 0, 181, 182, 183, 0, 0, 0, 187, 0, 189, 0,
    0, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
    254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    150, 151, 175, 0, 0, 145, 146, 130, 0, 147, 148, 132, 0, 134, 135, 149, 0,
    0, 0, 133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 0, 139,
    155, 0, 0, 0, 0, 0, 180, 161, 162, 0, 184, 185, 186, 0, 188, 0, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 0, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221,
    222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236,
    237, 238, 239,
]; // 392 entries

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 63, 0, 0, 0, 193, 0, 0, 0, 0, 0, 0, 0, 328, 127, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 268, 0, 147, 0, 223,
]; // 133 entries

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[cfg(not(feature = "no-optimized-legacy-encoding"))]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 6) as usize;
    let offset = if offset < 133 {BACKWARD_TABLE_UPPER[offset] as usize} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 63) as usize)]
}

/// Returns the index pointer for code point `code` in this index.
#[cfg(feature = "no-optimized-legacy-encoding")]
pub fn backward(code: u32) -> u8 {
    if code > 8482 || ((0x10003u32 >> (code >> 9)) & 1) == 0 { return 0; }
    let code = code as u16;
    for i in 0..0x80 {
        if FORWARD_TABLE[i as usize] == code { return 0x80 + i; }
    }
    0
}

#[cfg(test)]
single_byte_tests! {
}
