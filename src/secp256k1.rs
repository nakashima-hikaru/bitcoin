use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use primitive_types::U256;

use crate::{
    field_element::FieldElement,
    point::{PlaneElement, Point},
    signature::Signature,
};
use hex::FromHex;

pub const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
pub const GX: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
pub const GY: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
pub const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
pub const A: u32 = 0;
pub const B: u32 = 7;

#[derive(Debug, Clone, Copy)]
pub struct S256Field {
    field_element: FieldElement,
}

impl S256Field {
    pub fn new(num: U256) -> Self {
        let field_element = FieldElement::new(num, S256Field::get_prime());
        Self { field_element }
    }
    pub fn as_field_element(&self) -> FieldElement {
        self.field_element
    }

    pub fn get_inverse(&self) -> S256Field {
        S256Field::new(self.as_field_element().get_inverse().get_num())
    }

    pub fn get_num(&self) -> U256 {
        self.as_field_element().get_num()
    }

    fn get_prime() -> U256 {
        U256::from_str_radix(P, 16).unwrap()
    }

    pub fn sqrt(&self) -> U256 {
        return self
            .as_field_element()
            .pow((S256Field::get_prime() + U256::one()) / U256::from(4))
            .get_num();
    }
}

impl Add for S256Field {
    type Output = Self;
    fn add(self, rhs: S256Field) -> Self::Output {
        S256Field {
            field_element: self.field_element + rhs.field_element,
        }
    }
}

impl Sub for S256Field {
    type Output = Self;
    fn sub(self, rhs: S256Field) -> Self::Output {
        S256Field {
            field_element: self.field_element - rhs.field_element,
        }
    }
}

impl Mul for S256Field {
    type Output = Self;
    fn mul(self, rhs: S256Field) -> Self::Output {
        S256Field {
            field_element: self.field_element * rhs.field_element,
        }
    }
}

impl Div for S256Field {
    type Output = Self;
    fn div(self, rhs: S256Field) -> Self::Output {
        S256Field {
            field_element: self.field_element / rhs.field_element,
        }
    }
}

impl AddAssign for S256Field {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for S256Field {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<S256Field> for S256Field {
    fn mul_assign(&mut self, rhs: S256Field) {
        *self = *self * rhs;
    }
}

impl DivAssign<S256Field> for S256Field {
    fn div_assign(&mut self, rhs: S256Field) {
        *self = *self / rhs;
    }
}

impl fmt::Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.field_element.get_num())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct S256Point {
    point: Point,
}

impl S256Point {
    pub fn new(x: Option<U256>, y: Option<U256>) -> Self {
        let a = S256Field::new(U256::from(A)).field_element;
        let b = S256Field::new(U256::from(B)).field_element;
        let point;
        if let (Some(x_plane), Some(y_plane)) = (x, y) {
            let z = PlaneElement::new(
                S256Field::new(x_plane).as_field_element(),
                S256Field::new(y_plane).as_field_element(),
            );
            point = Point::new(Some(z), a, b);
        } else {
            point = Point::new(None, a, b);
        }
        Self { point }
    }

    pub fn as_point(&self) -> Point {
        self.point
    }

    pub fn get_the_generic_point() -> Self {
        S256Point::new(
            Some(U256::from_str_radix(GX, 16).unwrap()),
            Some(U256::from_str_radix(GY, 16).unwrap()),
        )
    }
    pub fn get_x(&self) -> FieldElement {
        self.point.get_coordinate().unwrap().get_x()
    }
    pub fn get_y(&self) -> FieldElement {
        self.point.get_coordinate().unwrap().get_y()
    }

    pub fn get_the_order_of_generic_point() -> U256 {
        U256::from_str_radix(N, 16).unwrap()
    }

    pub fn verify(&self, z: U256, sig: Signature) -> bool {
        let tmp = FieldElement::new(sig.get_s(), U256::from_str_radix(N, 16).unwrap());
        let s_inv = tmp.get_inverse().get_num();
        let u = U256::try_from(z.full_mul(s_inv) % N).unwrap();
        let v = U256::try_from(sig.get_r().full_mul(s_inv) % N).unwrap();
        let total = u * Self::get_the_generic_point() + v * *self;
        total.point.get_coordinate().unwrap().get_x().get_num() == sig.get_r()
    }

    /// * 非圧縮方式
    ///
    ///     ナイーブにPointのx座標・y座標をbig endianで16進数に変換してつなげる
    pub fn sec(&self) -> [u8; 65] {
        let mut ret: [u8; 65] = [b'\x00'; 65];
        ret[0] = b'\x04';
        let mut x_bytes: [u8; 32] = Default::default();
        self.get_x().get_num().to_big_endian(&mut x_bytes);
        let mut y_bytes: [u8; 32] = Default::default();
        self.get_y().get_num().to_big_endian(&mut y_bytes);
        for i in 0..32 {
            ret[i + 1] = x_bytes[i];
        }
        for i in 0..32 {
            ret[i + 33] = y_bytes[i];
        }
        ret
    }

    /// * 圧縮方式
    ///
    ///     yの偶奇とxを返す。
    ///     xに対応する二つのyの偶奇は異なるので、yの偶奇とxからyが復元できる。
    pub fn compressed_sec(&self) -> [u8; 33] {
        let mut ret: [u8; 33] = [b'\x00'; 33];
        if self.get_y().get_num().bit(0) {
            ret[0] = b'\x03';
        } else {
            ret[0] = b'\x02';
        }
        let mut x_bytes: [u8; 32] = Default::default();
        self.get_x().get_num().to_big_endian(&mut x_bytes);
        for i in 0..32 {
            ret[i + 1] = x_bytes[i];
        }
        ret
    }
}
impl Add for S256Point {
    type Output = Self;
    fn add(self, rhs: S256Point) -> Self::Output {
        Self {
            point: self.point + rhs.point,
        }
    }
}

impl Mul<S256Point> for U256 {
    type Output = S256Point;
    fn mul(self, rhs: S256Point) -> Self::Output {
        let coefficient = self % U256::from_str_radix(N, 16).unwrap();
        S256Point {
            point: coefficient * rhs.point,
        }
    }
}

impl Mul<U256> for S256Point {
    type Output = S256Point;
    fn mul(self, rhs: U256) -> Self::Output {
        rhs * self
    }
}

impl AddAssign for S256Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<U256> for S256Point {
    fn mul_assign(&mut self, rhs: U256) {
        self.point = self.point * rhs
    }
}

impl fmt::Display for S256Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.point.get_coordinate() == None {
            write!(f, "Identity")
        } else {
            write!(
                f,
                "({}, {})",
                self.point.get_coordinate().unwrap().get_x().get_num(),
                self.point.get_coordinate().unwrap().get_y().get_num(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify() {
        let point = S256Point::new(
            Some(
                U256::from_str_radix(
                    "0x887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                    16,
                )
                .unwrap(),
            ),
            Some(
                U256::from_str_radix(
                    "0x61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                    16,
                )
                .unwrap(),
            ),
        );
        let z = U256::from_str_radix(
            "0xec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();
        let r = U256::from_str_radix(
            "0xac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
            16,
        )
        .unwrap();
        let s = U256::from_str_radix(
            "0x68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            16,
        )
        .unwrap();
        assert!(point.verify(z, Signature::new(r, s)));
        let z = U256::from_str_radix(
            "0x7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();
        let r = U256::from_str_radix(
            "0xeff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            16,
        )
        .unwrap();
        let s = U256::from_str_radix(
            "0xc7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            16,
        )
        .unwrap();
        assert!(point.verify(z, Signature::new(r, s)));
    }

    #[test]
    fn test_sec() {
        fn test(secret: U256, expected_str: &str) {
            let public = (secret * S256Point::get_the_generic_point())
                .as_point()
                .get_coordinate()
                .unwrap();
            let tmp = S256Point::new(
                Some(public.get_x().get_num()),
                Some(public.get_y().get_num()),
            );
            let result = tmp.sec();
            let mut expected: [u8; 65] = [0; 65];
            hex::decode_to_slice(&expected_str, &mut expected).unwrap();
            assert_eq!(result, expected);
        }

        let secret = U256::from(5000);
        let expected_str =
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4\
f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10";
        test(secret, &expected_str);
        let secret = U256::from(2018).pow(U256::from(5));
        let expected_str =
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023\
c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06";
        test(secret, &expected_str);
        let secret = U256::from_str_radix("0xdeadbeef12345", 16).unwrap();
        let expected_str =
            "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd5\
9961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121";
        test(secret, &expected_str);
    }
}
