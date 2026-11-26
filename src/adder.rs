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
}
