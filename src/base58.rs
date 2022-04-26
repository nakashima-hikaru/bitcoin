use hmac::{Hmac, Mac};
use primitive_types::{U256, U512};
use sha2::{Digest, Sha256};
type HmacSha256 = Hmac<Sha256>;
use ripemd::Ripemd160;

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(b: &[u8]) -> String {
    let mut count = 0; // number of zeros
    for &c in b {
        if c == 0 {
            count += 1;
        } else {
            break;
        }
    }
    let mut num = U512::from_big_endian(&b);
    let mut prefix = "1".to_string().repeat(count);
    let mut result = "".to_string();
    while !num.is_zero() {
        let rem;
        (num, rem) = num.div_mod(U512::from(58));
        result = BASE58_ALPHABET
            .chars()
            .nth(usize::try_from(rem).unwrap())
            .unwrap()
            .to_string()
            + &result;
    }
    println!("{}", prefix);
    prefix.push_str(&result);
    prefix
}

pub fn hash256(b: &[u8]) -> Vec<u8> {
    Sha256::digest(Sha256::digest(b)).to_vec()
}

pub fn encode_base58_checksum(b: &[u8]) -> String {
    encode_base58(&[b, &hash256(b)[..4]].concat())
}

pub fn hash160(b: &[u8]) -> Vec<u8> {
    Ripemd160::digest(Sha256::digest(b)).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base58() {
        let s = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let mut ret: [u8; 32] = Default::default();
        hex::decode_to_slice(s, &mut ret).unwrap();
        let ret = encode_base58(&ret);
        assert_eq!(ret, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6")
    }
}
