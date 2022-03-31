use primitive_types::U256;

mod field_element;
mod point;

fn main() {
    let p = U256::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap();
    let gx = U256::from_str_radix(
        "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        16,
    )
    .unwrap();
    let gy = U256::from_str_radix(
        "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16,
    )
    .unwrap();
    let n = U256::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap();
    let gy = field_element::FieldElement::new(gy, p);
    let gx = field_element::FieldElement::new(gx, p);
    assert_eq!(
        gy * gy,
        gx * gx * gx + field_element::FieldElement::new(U256::from(7), p)
    );
}
