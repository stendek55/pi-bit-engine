#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdderResult {
    pub sum: bool,
    pub carry: bool,
}

pub fn full_adder(a: bool, b: bool, carry_in: bool) -> AdderResult {
    todo!("Full Adder implementation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adder_zero_zero_zero() {
        let result = full_adder(false, false, false);
        assert_eq!(result.sum, false);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn full_adder_one_zero_zero() {
        let result = full_adder(true, false, false);
        assert_eq!(result.sum, true);
        assert_eq!(result.carry, false);
    }

    #[test]
    fn full_adder_one_one_zero() {
        let result = full_adder(true, true, false);
        assert_eq!(result.sum, false);
        assert_eq!(result.carry, true);
    }

    #[test]
    fn full_adder_one_one_one() {
        let result = full_adder(true, true, true);
        assert_eq!(result.sum, true);
        assert_eq!(result.carry, true);
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
