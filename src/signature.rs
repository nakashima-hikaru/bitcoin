use crate::{
    field_element::FieldElement,
    secp256k1::{S256Point, N},
};
use hmac::{Hmac, Mac};
use primitive_types::U256;
use sha2::Digest;
use sha2::Sha256;
use std::fmt;
type HmacSha256 = Hmac<Sha256>;
pub struct Signature {
    r: FieldElement,
    s: FieldElement,
}

impl Signature {
    pub fn get_r(&self) -> FieldElement {
        self.r
    }
    pub fn get_s(&self) -> FieldElement {
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

    // pub fn deterministic_k(&self, mut z: U256) -> U256 {
    //     let mut k = b'\x00' * 32;
    //     let v = b'\x01' * 32;
    //     let n = U256::from_str_radix(N, 16).unwrap();
    //     if z > n {
    //         z -= n;
    //         let mut z_bytes: [u8; 32] = Default::default();
    //         let mut secret_bytes: [u8; 32] = Default::default();
    //         z.to_big_endian(&mut z_bytes);
    //         self.secret.to_big_endian(&mut secret_bytes);
    //         let mut s256 = Sha256::new();
    //         let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    //             .expect("HMAC can take key of any size");
    //         mac.update(b"input message");
    //         let tmp = HmacSha256::
    //     }
    //     U256::one()
    // }

    // pub fn sign(&self, z: U256) -> Signature {
    //     let mut rng = rand::thread_rng();
    //     let k = FieldElement::new(rng.gen::<U256>(), U256::from_str_radix(N, 16));
    // }
}
