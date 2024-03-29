use crate::{
    base58::encode_base58_checksum,
    field_element::FieldElement,
    secp256k1::{S256Point, N},
};
use hmac::{Hmac, Mac};
use primitive_types::U256;
use sha2::Sha256;
use std::{collections::VecDeque, fmt};
type HmacSha256 = Hmac<Sha256>;
pub struct Signature {
    r: U256,
    s: U256,
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Self { r, s }
    }
    pub fn get_r(&self) -> U256 {
        self.r
    }
    pub fn get_s(&self) -> U256 {
        self.s
    }
    pub fn der(&self) -> Vec<u8> {
        let mut ret: VecDeque<u8> = Default::default();
        let mut r_bytes: [u8; 32] = Default::default();
        self.r.to_big_endian(&mut r_bytes);
        let mut i = 0;
        while i < r_bytes.len() && r_bytes[i] == b'\x00' {
            i += 1;
        }
        if i < r_bytes.len() {
            // 2^7 = \x80
            let is_negative = r_bytes[i] & b'\x80' == b'\x80';
            if is_negative {
                ret.push_back(b'\x00');
            }
            for j in i..r_bytes.len() {
                ret.push_back(r_bytes[j]);
            }
            ret.push_front((r_bytes.len() + is_negative as usize) as u8);
        }
        ret.push_front(2 as u8);

        let mut ret2: VecDeque<u8> = Default::default();

        let mut s_bytes: [u8; 32] = Default::default();
        self.s.to_big_endian(&mut s_bytes);
        let mut i = 0;
        while i < s_bytes.len() && s_bytes[i] == b'\x00' {
            i += 1;
        }
        if i < s_bytes.len() {
            // 2^7 = \x80
            let is_negative = s_bytes[i] & b'\x80' == b'\x80';
            if is_negative {
                ret2.push_back(b'\x00');
            }
            for j in i..s_bytes.len() {
                ret2.push_back(s_bytes[j]);
            }
            ret2.push_front((s_bytes.len() + is_negative as usize) as u8);
        }
        ret2.push_front(2 as u8);
        ret.append(&mut ret2);
        ret.push_front(ret.len() as u8);
        ret.push_front(b'\x30');
        ret.into()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({},{})", self.r, self.s)
    }
}
pub struct PrivateKey {
    secret: U256,
    point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: U256) -> Self {
        PrivateKey {
            secret,
            point: secret * S256Point::get_the_generic_point(),
        }
    }

    pub fn sign(&self, z: U256) -> Signature {
        let k = self.deterministic_k(z);
        let r = (k * S256Point::get_the_generic_point())
            .as_point()
            .get_coordinate()
            .unwrap()
            .get_x()
            .get_num();
        let n = U256::from_str_radix(N, 16).unwrap();
        let k_inv = FieldElement::new(k % n, n).get_inverse();
        let s = (FieldElement::new(z, n) + r * FieldElement::new(self.secret, n)) * k_inv;
        let mut s = s.get_num();
        if s > n / U256::from(2) {
            s = n - s;
        }
        Signature::new(r, s)
    }

    pub fn deterministic_k(&self, z: U256) -> U256 {
        let mut k = [b'\x00'; 32];
        let mut v = [b'\x01'; 32];
        let n = U256::from_str_radix(N, 16).unwrap();
        let z = if z > n { z - n } else { z };
        let mut z_bytes: [u8; 32] = Default::default();
        let mut secret_bytes: [u8; 32] = Default::default();
        z.to_big_endian(&mut z_bytes);
        self.secret.to_big_endian(&mut secret_bytes);

        k = HmacSha256::new_from_slice(&k)
            .unwrap()
            .chain_update(&v)
            .chain_update(&[b'\x00'])
            .chain_update(&secret_bytes)
            .chain_update(&z_bytes)
            .finalize()
            .into_bytes()
            .try_into()
            .unwrap();

        v = HmacSha256::new_from_slice(&k)
            .unwrap()
            .chain_update(&v)
            .finalize()
            .into_bytes()
            .try_into()
            .unwrap();

        k = HmacSha256::new_from_slice(&k)
            .unwrap()
            .chain_update(&v)
            .chain_update(&[b'\x01'])
            .chain_update(&secret_bytes)
            .chain_update(&z_bytes)
            .finalize()
            .into_bytes()
            .try_into()
            .unwrap();

        v = HmacSha256::new_from_slice(&k)
            .unwrap()
            .chain_update(&v)
            .finalize()
            .into_bytes()
            .try_into()
            .unwrap();
        loop {
            v = HmacSha256::new_from_slice(&k)
                .unwrap()
                .chain_update(&v)
                .finalize()
                .into_bytes()
                .try_into()
                .unwrap();
            let candidate = U256::from(v.as_slice());
            if candidate > U256::one() && candidate < n {
                return candidate;
            }
            k = HmacSha256::new_from_slice(&k)
                .unwrap()
                .chain_update(&v)
                .chain_update(&[b'\x00'])
                .finalize()
                .into_bytes()
                .try_into()
                .unwrap();
            v = HmacSha256::new_from_slice(&k)
                .unwrap()
                .chain_update(&v)
                .finalize()
                .into_bytes()
                .try_into()
                .unwrap();
        }
    }

    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let mut secret_bytes: [u8; 32] = Default::default();
        self.secret.to_big_endian(&mut secret_bytes);
        let mut ret: Vec<u8> = Default::default();
        let prefix;
        if testnet {
            prefix = b'\xef';
        } else {
            prefix = b'\x80';
        }
        ret.append(&mut [prefix].to_vec());
        ret.append(&mut secret_bytes.to_vec());
        let suffix;
        if compressed {
            suffix = b'\x01';
            ret.append(&mut [suffix].to_vec());
        }
        encode_base58_checksum(&ret)
    }
}

#[cfg(test)]
mod tests {
    use hex::ToHex;

    use super::*;
    #[test]
    fn test_sign() {
        let pk = PrivateKey::new(U256::one());
        let z = U256::one();
        let sig = pk.sign(z);
        assert_eq!(
            pk.deterministic_k(z),
            U256::from_dec_str(
                "69770345078884640739184711464744623257826325099242396410478198115888237352364"
            )
            .unwrap()
        );
        assert!(pk.point.verify(z, sig));
    }

    #[test]
    fn test_der() {
        let r = U256::from_str_radix(
            "0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s = U256::from_str_radix(
            "0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();
        let sig = Signature::new(r, s);
        assert_eq!(
            hex::encode(sig.der()),
            "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6022100\
8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
        );
    }

    #[test]
    fn test_wif() {
        let key = PrivateKey::new(U256::from(5003));
        let ret = key.wif(true, true);
        assert_eq!(ret, "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK");

        let key = PrivateKey::new(U256::from(2021_i64.pow(5)));
        let ret = key.wif(false, true);
        assert_eq!(ret, "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic");

        let key = PrivateKey::new(U256::from_str_radix("0x54321deadbeef", 16).unwrap());
        let ret = key.wif(true, false);
        assert_eq!(ret, "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a");
    }
}
