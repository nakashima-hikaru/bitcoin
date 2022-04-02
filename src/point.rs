use std::{
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign},
};

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
    pub fn get_x(&self) -> FieldElement {
        self.x
    }
    pub fn get_y(&self) -> FieldElement {
        self.y
    }
}

impl fmt::Display for PlaneElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) mod {}",
            self.x.get_num(),
            self.y.get_num(),
            self.x.get_prime()
        )
    }
}

pub type SphereElement = Option<PlaneElement>;

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
    pub fn get_coordinate(&self) -> SphereElement {
        self.z
    }
    pub fn get_a(&self) -> FieldElement {
        self.a
    }
    pub fn get_b(&self) -> FieldElement {
        self.b
    }
    pub fn get_prime(&self) -> U256 {
        self.a.get_prime()
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

impl Mul<Point> for U256 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let mut ret = Point::new(None, rhs.a, rhs.b);
        let mut tmp_num = rhs;
        let mut tmp_exponent = self;
        // println!("{}", tmp_exponent);
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

impl Mul<U256> for Point {
    type Output = Point;
    fn mul(self, rhs: U256) -> Self::Output {
        let mut ret = Point::new(None, self.a, self.b);
        let mut tmp_num = self;
        let mut tmp_exponent = rhs;
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

impl MulAssign<U256> for Point {
    fn mul_assign(&mut self, rhs: U256) {
        *self = *self * rhs;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.z == None {
            write!(
                f,
                "Identity on y^2=x^3+{}x+{} mod {}",
                self.a.get_num(),
                self.b.get_num(),
                self.a.get_prime()
            )
        } else {
            write!(
                f,
                "({}, {}) on y^2=x^3+{}x+{} mod {}",
                self.z.unwrap().x.get_num(),
                self.z.unwrap().y.get_num(),
                self.a.get_num(),
                self.b.get_num(),
                self.a.get_prime()
            )
        }
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
        let x1 = FieldElement::new(U256::from(170), prime);
        let y1 = FieldElement::new(U256::from(142), prime);
        let z1 = PlaneElement::new(x1, y1);
        let x2 = FieldElement::new(U256::from(60), prime);
        let y2 = FieldElement::new(U256::from(139), prime);
        let z2 = PlaneElement::new(x2, y2);
        let p1 = Point::new(Some(z1), a, b);
        let p2 = Point::new(Some(z2), a, b);
        let x_ans = FieldElement::new(U256::from(220), prime);
        let y_ans = FieldElement::new(U256::from(181), prime);
        let z_ans = PlaneElement::new(x_ans, y_ans);
        assert_eq!(p1 + p2, Point::new(Some(z_ans), a, b));
        let mut p3 = p1;
        p3 += p2;
        assert_eq!(p3, Point::new(Some(z_ans), a, b));

        let x1 = FieldElement::new(U256::from(47), prime);
        let y1 = FieldElement::new(U256::from(71), prime);
        let z1 = PlaneElement::new(x1, y1);
        let x2 = FieldElement::new(U256::from(17), prime);
        let y2 = FieldElement::new(U256::from(56), prime);
        let z2 = PlaneElement::new(x2, y2);
        let p1 = Point::new(Some(z1), a, b);
        let p2 = Point::new(Some(z2), a, b);
        let x_ans = FieldElement::new(U256::from(215), prime);
        let y_ans = FieldElement::new(U256::from(68), prime);
        let z_ans = PlaneElement::new(x_ans, y_ans);
        assert_eq!(p1 + p2, Point::new(Some(z_ans), a, b));
        let mut p3 = p1;
        p3 += p2;
        assert_eq!(p3, Point::new(Some(z_ans), a, b));

        let x1 = FieldElement::new(U256::from(143), prime);
        let y1 = FieldElement::new(U256::from(98), prime);
        let z1 = PlaneElement::new(x1, y1);
        let x2 = FieldElement::new(U256::from(76), prime);
        let y2 = FieldElement::new(U256::from(66), prime);
        let z2 = PlaneElement::new(x2, y2);
        let p1 = Point::new(Some(z1), a, b);
        let p2 = Point::new(Some(z2), a, b);
        let x_ans = FieldElement::new(U256::from(47), prime);
        let y_ans = FieldElement::new(U256::from(71), prime);
        let z_ans = PlaneElement::new(x_ans, y_ans);
        assert_eq!(p1 + p2, Point::new(Some(z_ans), a, b));
        let mut p3 = p1;
        p3 += p2;
        assert_eq!(p3, Point::new(Some(z_ans), a, b));
    }

    #[test]
    fn test_mul() {
        let prime = U256::from(223);
        let a = FieldElement::new(U256::from(0), prime);
        let b = FieldElement::new(U256::from(7), prime);
        let x = FieldElement::new(U256::from(192), prime);
        let y = FieldElement::new(U256::from(105), prime);
        let z = PlaneElement::new(x, y);
        let p = Point::new(Some(z), a, b);
        let n = U256::from(2);
        let x_ans = FieldElement::new(U256::from(49), prime);
        let y_ans = FieldElement::new(U256::from(71), prime);
        let z_ans = PlaneElement::new(x_ans, y_ans);
        let p_ans = Point::new(Some(z_ans), a, b);
        assert_eq!(U256::one() * p, p);
        assert_eq!(n * p, p + p);
        assert_eq!(n * p, p_ans);
    }
}
