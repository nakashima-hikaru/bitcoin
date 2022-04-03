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
        let field_element = FieldElement::new(num, U256::from_str_radix(P, 16).unwrap());
        Self { field_element }
    }
    pub fn as_field_element(&self) -> FieldElement {
        self.field_element
    }
    pub fn get_inverse(&self) -> S256Field {
        S256Field::new(self.as_field_element().get_inverse().get_num())
    }
    pub fn get_num(self) -> U256 {
        self.as_field_element().get_num()
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

    pub fn get_generic_point() -> Self {
        S256Point::new(
            Some(U256::from_str_radix(GX, 16).unwrap()),
            Some(U256::from_str_radix(GY, 16).unwrap()),
        )
    }
    pub fn get_order_of_generic_point() -> U256 {
        U256::from_str_radix(N, 16).unwrap()
    }
    pub fn verify(&self, z: U256, sig: Signature) -> bool {
        let tmp = FieldElement::new(sig.get_s().get_num(), U256::from_str_radix(N, 16).unwrap());
        let s_inv = tmp.get_inverse();
        let u = (z * s_inv).get_num();
        let v = (sig.get_r() * s_inv).get_num();
        let total = u * Self::get_generic_point() + v * *self;
        total.point.get_coordinate().unwrap().get_x() == sig.get_r()
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

#[test]
fn main() {
    let g = S256Point::get_generic_point();
    let n = S256Point::get_order_of_generic_point();
    println!("{}", n * g);
}
