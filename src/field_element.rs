use core::fmt::Debug;
use primitive_types::U256;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldElement {
    num: U256,
    prime: U256,
}

impl FieldElement {
    pub fn new(num: U256, prime: U256) -> FieldElement {
        if num >= prime {
            let error = format!(
                "Num {:?} not in field range 0 to {:?}",
                num,
                prime - U256::one()
            );
            panic!("{}", error);
        }
        FieldElement { num, prime }
    }
    pub fn get_num(&self) -> U256 {
        self.num
    }
    pub fn get_prime(&self) -> U256 {
        self.prime
    }

    pub fn pow(self, exponent: U256) -> FieldElement {
        let mut ret = FieldElement::new(U256::one(), self.prime);
        let mut tmp_num = self;
        let mut tmp_exponent = exponent;
        while !tmp_exponent.is_zero() {
            if tmp_exponent.bit(0) {
                ret *= tmp_num;
            }
            tmp_num *= tmp_num;
            tmp_exponent >>= 1;
        }
        ret
    }

    pub fn get_inverse(&self) -> FieldElement {
        self.pow(self.prime - U256::from(2))
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        let num;
        if self.prime - self.num <= rhs.num {
            num = self.prime - (self.prime - self.num) - (self.prime - rhs.num);
        } else {
            num = rhs.num + self.num;
        }
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        let tmp;
        if self.num >= rhs.num {
            tmp = self.num - rhs.num;
        } else {
            tmp = (self.prime - rhs.num) + self.num;
        }
        let num = tmp % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = Self;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        let num = (self.num.full_mul(rhs.num)) % self.prime;
        FieldElement {
            num: U256::try_from(num).unwrap(),
            prime: self.prime,
        }
    }
}

impl Mul<U256> for FieldElement {
    type Output = Self;
    fn mul(self, rhs: U256) -> Self::Output {
        self * FieldElement::new(rhs % self.prime, self.prime)
    }
}

impl Mul<FieldElement> for U256 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        rhs * self
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(self.prime, rhs.prime);
        self * rhs.pow(self.prime - U256::from(2))
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for FieldElement {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: FieldElement) {
        *self = *self * rhs;
    }
}

impl MulAssign<U256> for FieldElement {
    fn mul_assign(&mut self, rhs: U256) {
        *self = *self * rhs;
    }
}

impl DivAssign<FieldElement> for FieldElement {
    fn div_assign(&mut self, rhs: FieldElement) {
        *self = *self / rhs;
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  mod {}", self.num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_add() {
        let prime = U256::from(57);
        let fp = FieldElement::new(U256::from(44), prime);
        let fp2 = FieldElement::new(U256::from(33), prime);
        assert_eq!(fp + fp2, FieldElement::new(U256::from(20), prime));

        let fp = FieldElement::new(U256::from(9), prime);
        let fp2 = FieldElement::new(U256::from(prime - 29), prime);
        assert_eq!(fp + fp2, FieldElement::new(U256::from(37), prime));
    }

    #[test]
    fn test_mul() {
        let prime = U256::from(97);
        let fp = FieldElement::new(U256::from(95), prime);
        let fp2 = FieldElement::new(U256::from(45), prime);
        let fp3 = FieldElement::new(U256::from(31), prime);
        assert_eq!(fp * fp2 * fp3, FieldElement::new(U256::from(23), prime));

        let fp = FieldElement::new(U256::from(17), prime);
        let fp2 = FieldElement::new(U256::from(13), prime);
        let fp3 = FieldElement::new(U256::from(19), prime);
        let fp4 = FieldElement::new(U256::from(44), prime);
        assert_eq!(
            fp * fp2 * fp3 * fp4,
            FieldElement::new(U256::from(68), prime)
        );
        let fp = FieldElement::new(U256::from(12), prime);
        let fp2 = FieldElement::new(U256::from(77), prime);
        assert_eq!(
            fp.pow(U256::from(7)) * fp2.pow(U256::from(49)),
            FieldElement::new(U256::from(63), prime)
        );
    }

    #[test]
    fn test_overflow() {
        let prime = U256::from_dec_str(
            "115792089237316195423570985008687907853269984665640564039457584007908834671663",
        )
        .unwrap();
        let fp = FieldElement::new(prime - 1, prime);
        let fp2 = FieldElement::new(prime - 1, prime);
        assert_eq!((fp + fp2).get_num(), prime - U256::from(2));
        assert_eq!((fp - fp2).get_num(), U256::from(0));
        assert_eq!((fp * fp2).get_num(), U256::from(1));
        assert_eq!((fp / fp2).get_num(), U256::from(1));

        let fp = FieldElement::new(U256::from(1), prime);
        assert_eq!((fp + fp2).get_num(), U256::from(0));
        assert_eq!((fp - fp2).get_num(), U256::from(2));
        assert_eq!((fp * fp2), fp2);
        assert_eq!((fp / fp2), fp2);
    }

    #[bench]
    fn bench_multiple1(b: &mut Bencher) {
        b.iter(|| test_overflow());
    }
}
