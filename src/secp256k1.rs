use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use primitive_types::U256;

use crate::{
    field_element::FieldElement,
    point::{PlaneElement, Point, SphereElement},
};

const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
const GX: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const GY: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
const A: u32 = 0;
const B: u32 = 7;

#[derive(Debug, Clone, Copy)]
struct S256Field {
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
}

#[derive(Debug, Clone, Copy)]
struct S256Point {
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
        S256Point {
            point: self * rhs.point,
        }
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
