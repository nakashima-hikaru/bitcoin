use crate::{
    field_element::FieldElement,
    secp256k1::{S256Point, N},
};
use hmac::{Hmac, Mac};
use primitive_types::U256;
use sha2::Sha256;
use std::fmt;
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
            point: secret * S256Point::get_generic_point(),
        }
    }

    pub fn sign(&self, z: U256) -> Signature {
        let k = self.deterministic_k(z);
        let r = (k * S256Point::get_generic_point())
            .get_point()
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

    pub fn deterministic_k(&self, mut z: U256) -> U256 {
        let mut k = vec![b'\x00'; 32];
        let mut v = vec![b'\x01'; 32];
        let n = U256::from_str_radix(N, 16).unwrap();
        if z > n {
            z -= n;
        }
        let mut z_bytes: [u8; 32] = Default::default();
        let mut secret_bytes: [u8; 32] = Default::default();
        z.to_big_endian(&mut z_bytes);
        self.secret.to_big_endian(&mut secret_bytes);
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        hmac.update(&[b'\x00']);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);
        k = hmac.finalize().into_bytes().as_slice().to_vec();
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        v = hmac.finalize().into_bytes().as_slice().to_vec();

        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        hmac.update(&[b'\x01']);
        hmac.update(&secret_bytes);
        hmac.update(&z_bytes);
        k = hmac.finalize().into_bytes().as_slice().to_vec();
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        v = hmac.finalize().into_bytes().as_slice().to_vec();
        loop {
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().as_slice().to_vec();
            let candidate = U256::from(v.as_slice());
            if candidate > U256::one() && candidate < n {
                return candidate;
            }
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            hmac.update(&[b'\x00']);
            k = hmac.finalize().into_bytes().as_slice().to_vec();
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().as_slice().to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
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
}
