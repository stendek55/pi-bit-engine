use crate::BitByte;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdderResult {
    pub sum: bool,
    pub carry: bool,
}

pub fn full_adder(a: bool, b: bool, carry_in: bool) -> AdderResult {
    let a_bit = BitByte::new(u8::from(a));
    let b_bit = BitByte::new(u8::from(b));
    let carry_bit = BitByte::new(u8::from(carry_in));

    let partial_sum = a_bit.bitwise_xor(b_bit);

    let sum = partial_sum.bitwise_xor(carry_bit);

    let carry = a_bit
        .bitwise_and(b_bit)
        .bitwise_or(partial_sum.bitwise_and(carry_bit));

    AdderResult {
        sum: sum.check_bit(0).expect("bit 0 must be valid"),
        carry: carry.check_bit(0).expect("bit 0 must be valid"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitAdder8Result {
    pub sum: BitByte,
    pub carry: bool,
}

pub fn bit_adder_8(a: u8, b: u8) -> BitAdder8Result {
    let a_byte = BitByte::new(a);
    let b_byte = BitByte::new(b);

    let mut sum = BitByte::new(0);
    let mut carry = false;

    for index in 0..8 {
        let a_bit = a_byte.check_bit(index).expect("index must be valid");

        let b_bit = b_byte.check_bit(index).expect("index must be valid");

        let result = full_adder(a_bit, b_bit, carry);

        if result.sum {
            sum = sum.set_bit(index).expect("index must be valid");
        }

        carry = result.carry;
    }

    BitAdder8Result { sum, carry }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adder_zero_zero_zero() {
        let result = full_adder(false, false, false);
        assert!(!result.sum);
        assert!(!result.carry);
    }

    #[test]
    fn full_adder_one_zero_zero() {
        let result = full_adder(true, false, false);
        assert!(result.sum);
        assert!(!result.carry);
    }

    #[test]
    fn full_adder_one_one_zero() {
        let result = full_adder(true, true, false);
        assert!(!result.sum);
        assert!(result.carry);
    }

    #[test]
    fn full_adder_one_one_one() {
        let result = full_adder(true, true, true);
        assert!(result.sum);
        assert!(result.carry);
    }

    #[test]
    fn full_adder_exhaustive() {
        for a in [false, true] {
            for b in [false, true] {
                for carry_in in [false, true] {
                    let result = full_adder(a, b, carry_in);

                    let reference = a as u8 + b as u8 + carry_in as u8;

                    assert_eq!(result.sum, reference & 1 == 1);
                    assert_eq!(result.carry, reference >= 2);
                }
            }
        }
    }

    #[test]
    fn bit_adder_8_zero() {
        let result = bit_adder_8(0, 0);

        assert_eq!(result.sum.value(), 0);
        assert!(!result.carry);
    }

    #[test]
    fn bit_adder_8_simple() {
        let result = bit_adder_8(5, 3);

        assert_eq!(result.sum.value(), 8);
        assert!(!result.carry);
    }

    #[test]
    fn bit_adder_8_carry() {
        let result = bit_adder_8(255, 1);

        assert_eq!(result.sum.value(), 0);
        assert!(result.carry);
    }

    #[test]
    fn bit_adder_8_overflow_boundary() {
        let result = bit_adder_8(200, 100);

        assert_eq!(result.sum.value(), 44);
        assert!(result.carry);
    }

    #[test]
    fn bit_adder_8_exhaustive() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                let result = bit_adder_8(a, b);
                let reference = u16::from(a) + u16::from(b);

                assert_eq!(
                    result.sum.value(),
                    reference as u8,
                    "sum mismatch: {a} + {b}"
                );

                assert_eq!(result.carry, reference > 255, "carry mismatch: {a} + {b}");
            }
        }
    }
}
