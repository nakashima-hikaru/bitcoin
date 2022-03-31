use std::ops::{Add, AddAssign, Mul};

use crate::field_element::FieldElement;
use primitive_types::U256;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct PlaneElement {
    x: FieldElement,
    y: FieldElement,
}

impl PlaneElement {
    pub fn new(x: FieldElement, y: FieldElement) -> PlaneElement {
        PlaneElement { x, y }
    }
}

type SphereElement = Option<PlaneElement>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    z: SphereElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(z: SphereElement, a: FieldElement, b: FieldElement) -> Self {
        let ret = Self { z, a, b };
        match z {
            None => (),
            _ => {
                assert_eq!(
                    z.unwrap().y.pow(U256::from(2)),
                    z.unwrap().x.pow(U256::from(3)) + a * z.unwrap().x + b
                )
            }
        };
        ret
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        if (self.a != rhs.a) || (self.b != rhs.b) {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }
        if self.z == None {
            return rhs;
        }
        if rhs.z == None {
            return self;
        }
        let sz = self.z.unwrap();
        let rz = rhs.z.unwrap();

        if self == rhs && sz.y.get_num() == U256::zero() {
            return Self {
                z: None,
                a: self.a,
                b: self.b,
            };
        } else if self == rhs {
            let s = (FieldElement::new(U256::from(3), self.a.get_prime()) * sz.x * sz.x + self.a)
                / (FieldElement::new(U256::from(2), self.a.get_prime()) * sz.y);
            let x = s * s - FieldElement::new(U256::from(2), self.a.get_prime()) * sz.x;
            let y = s * (sz.x - x) - sz.y;
            return Point {
                z: Some(PlaneElement::new(x, y)),
                a: self.a,
                b: self.b,
            };
        } else if sz.x == rz.x {
            return Self {
                z: None,
                a: self.a,
                b: self.b,
            };
        }
        let s = (rz.y - sz.y) / (rz.x - sz.x);
        let x = s * s - sz.x - rz.x;
        let y = s * (sz.x - x) - sz.y;
        Point {
            z: Some(PlaneElement::new(x, y)),
            a: self.a,
            b: self.b,
        }
    }
}

impl Mul<U256> for Point {
    type Output = Self;
    fn mul(self, exponent: U256) -> Self::Output {
        let mut ret = self;
        let mut tmp_num = self;
        let mut tmp_exponent = exponent;
        while tmp_exponent > U256::zero() {
            if tmp_exponent & U256::one() == U256::one() {
                ret += tmp_num;
            }
            tmp_num += tmp_num;
            tmp_exponent >>= 1;
        }
        ret
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let prime = U256::from(223);
        let a = FieldElement::new(U256::from(0), prime);
        let b = FieldElement::new(U256::from(7), prime);
        let x1 = FieldElement::new(U256::from(192), prime);
        let y1 = FieldElement::new(U256::from(105), prime);
        let z1 = PlaneElement::new(x1, y1);
        let x2 = FieldElement::new(U256::from(17), prime);
        let y2 = FieldElement::new(U256::from(56), prime);
        let z2 = PlaneElement::new(x2, y2);
        let p1 = Point::new(Some(z1), a, b);
        let p2 = Point::new(Some(z2), a, b);
        println!("{:?}", p1 + p2);
    }
}
