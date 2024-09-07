/* city.rs --- CITY

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Some primes between 2^63 and 2^64 for various uses.
const K0: u64 = 0xc3a5c85c97cb3127;
const K1: u64 = 0xb492b66fbe98f273;
const K2: u64 = 0x9ae16a3b2f90404f;
const K3: u64 = 0xc949d7c7509e6557;

pub struct Uint128 {
    low: u64,
    high: u64,
}

/// Hash function for a byte array.
#[inline]
fn city_hash64(buf: &[u8]) -> u64 {
    todo!()
}

/// Hash function for a byte array.  For convenience, a 64-bit_vec seed is also
/// hashed into the result.
#[inline]
fn city_hash64_with_seed(buf: &[u8], seed: u64) -> u64 {
    todo!()
}

/// Hash function for a byte array.  For convenience, two seeds are also
/// hashed into the result.
#[inline]
fn city_hash64_with_seeds(buf: &[u8], seed0: u64, seed1: u64) -> u64 {
    todo!()
}

/// Hash function for a byte array.
#[inline]
fn city_hash128(buf: &[u8]) -> Uint128 {
    todo!()
}

/// Hash function for a byte array.  For convenience, a 128-bit_vec seed is also
/// hashed into the result.
#[inline]
fn city_hash128_with_seed(buf: &[u8], seed: u128) -> Uint128 {
    todo!()
}

/// Hash 128 input bits down to 64 bits of output.
/// This is intended to be a reasonably good hash function.
#[inline]
fn hash128to64(x: Uint128) -> u64 {
    // Murmur-inspired hashing.
    let k_mul: u64 = 0x9ddfea08eb382d69;
    let mut a: u64 = x.high ^ x.low * k_mul as u64;
    a ^= a >> 47;
    let mut b: u64 = (x.high ^ a) * k_mul;
    b ^= b >> 47;
    b *= k_mul;
    b
}

/// Bitwise right rotate.  Normally this will compile to a single
/// instruction, especially if the shift is a manifest constant.
fn rotate(val: u64, shift: isize) -> u64 {
    // Avoid shifting by 64: doing so yields an undefined result.
    match shift {
        0 => val,
        _ => (val >> shift) | (val << (64 - shift)),
    }
}

/// Equivalent to rotate(), but requires the second arg to be non-zero.
/// On x86-64, and probably others, it's possible for this to compile
/// to a single instruction if both args are already in registers.
fn rotate_by_at_least1(val: u64, shift: isize) -> u64 {
    (val >> shift) | (val << (64 - shift))
}

fn shift_mix(val: u64) -> u64 {
    val ^ (val >> 47)
}

fn hash_len16(u: u64, v: u64) -> u64 {
    let result = Uint128 { low: u, high: v };
    hash128to64(result)
}

fn unaligned_load64(p: &[u8]) -> u64 {
    let bytes: [u8; 8] = p[..8].try_into().expect("Slice with incorrect length");
    u64::from_ne_bytes(bytes)
}

fn unaligned_load32(p: &[u8]) -> u32 {
    let bytes: [u8; 4] = p[..4].try_into().expect("Slice with incorrect length");
    u32::from_ne_bytes(bytes)
}

#[cfg(target_endian = "little")]
fn uint32_in_expected_order(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "little")]
fn uint64_in_expected_order(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "big")]
fn uint32_in_expected_order(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
fn uint64_in_expected_order(x: u64) -> u64 {
    x.swap_bytes()
}

fn fetch64(p: &[u8]) -> u64 {
    uint64_in_expected_order(unaligned_load64(p))
}

fn fetch32(p: &[u8]) -> u32 {
    uint32_in_expected_order(unaligned_load32(p))
}

fn hash_len0to16(s: &[u8], len: usize) -> u64 {
    if len > 8 {
        let a = fetch64(s);
        let b = fetch64(&s[(len - 8)..]);
        return hash_len16(a, rotate_by_at_least1(b + len as u64, len as isize)) ^ b;
    }
    if len >= 4 {
        let a: u64 = fetch32(s) as u64;
        return hash_len16(len as u64 + (a << 3), fetch32(&s[(len - 4)..]) as u64);
    }
    if len > 0 {
        let a = s[0];
        let b = s[len >> 1];
        let c = s[len - 1];
        let y: u32 = (a as u32) + ((b as u32) << 8);
        let z: u32 = len as u32 + ((c as u32) << 2);
        return shift_mix(y as u64 * K2 ^ z as u64 * K3) * K2;
    }

    K2
}

/// This probably works well for 16-byte strings as well, but it may be overkill
/// in that case.
fn hash_len17to32(s: &[u8], len: usize) -> u64 {
    let a: u64 = fetch64(s) * K1;
    let b: u64 = fetch64(&s[8..]);
    let c: u64 = fetch64(&s[(len - 8)..]) * K2;
    let d: u64 = fetch64(&s[(len - 16)..]) * K0;
    hash_len16(
        rotate(a - b, 43) + rotate(c, 30) + d,
        a + rotate(b ^ K3, 20) - c + len as u64,
    )
}

/// Return a 16-byte hash for 48 bytes.  Quick and dirty.
/// Callers do best to use "random-looking" values for a and b.
pub fn weak_hash_len32_with_seeds6(w: u64, x: u64, y: u64, z: u64, a: u64, b: u64) -> Uint128 {
    let mut a = a + w;
    let mut b = rotate(b + a + z, 21);
    let c = a;
    a += x;
    a += y;
    b += rotate(a, 44);

    Uint128 {
        low: a + z,
        high: b + c,
    }
}

/// Return a 16-byte hash for s[0] ... s[31], a, and b.  Quick and dirty.
pub fn weak_hash_len32_with_seeds(s: &[u8], a: u64, b: u64) -> Uint128 {
    weak_hash_len32_with_seeds6(
        fetch64(s),
        fetch64(&s[8..]),
        fetch64(&s[16..]),
        fetch64(&s[24..]),
        a,
        b,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash128to64_zero() {
        let input = Uint128 { high: 0, low: 0 };
        let result = hash128to64(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_hash128to64_simple() {
        let input = Uint128 {
            high: 12345678,
            low: 87654321,
        };
        let result = hash128to64(input);
        assert_eq!(result, 14224548779526731012);
    }

    #[test]
    fn test_hash128to64_max_values() {
        let input = Uint128 {
            high: u64::MAX,
            low: u64::MAX,
        };
        let result = hash128to64(input);
        assert_eq!(result, 17822925301445087585);
    }

    #[test]
    fn test_hash128to64_high_zero_low_max() {
        let input = Uint128 {
            high: 0,
            low: u64::MAX,
        };
        let result = hash128to64(input);
        assert_eq!(result, 11040400438045666811);
    }

    #[test]
    fn test_hash128to64_low_zero_high_max() {
        let input = Uint128 {
            high: u64::MAX,
            low: 0,
        };
        let result = hash128to64(input);
        assert_eq!(result, 2920975488477388140);
    }
}
/* city.rs ends here */
