use crate::BitByte;

pub struct BitSubtractor8Result {
    pub difference: BitByte,
    pub borrow: bool,
}

pub struct BitSubtractor8 {}

impl BitSubtractor8 {
    pub fn subtract(a: u8, b: u8) -> BitSubtractor8Result {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract_simple_values_without_borrow() {
        // 10 - 3 = 7
        // Das Ergebnis passt in ein Byte und benötigt
        // keinen finalen Borrow.
        let result = BitSubtractor8::subtract(10, 3);

        assert_eq!(result.difference.value(), 7);
        assert!(!result.borrow);
    }

    #[test]
    fn subtract_equal_values_returns_zero() {
        // Jede Zahl minus sich selbst ergibt 0.
        // Es darf kein Borrow übrig bleiben.
        let result = BitSubtractor8::subtract(42, 42);

        assert_eq!(result.difference.value(), 0);
        assert!(!result.borrow);
    }

    #[test]
    fn subtract_requires_internal_borrow() {
        // 8 - 1 = 7
        //
        // Binär:
        // 00001000
        // 00000001
        //
        // Die unteren Bits benötigen ein Borrow,
        // das durch die Bitpositionen weitergegeben wird.
        let result = BitSubtractor8::subtract(8, 1);

        assert_eq!(result.difference.value(), 7);
        assert!(!result.borrow);
    }

    #[test]
    fn subtract_underflow_returns_final_borrow() {
        // 3 - 5 ist mathematisch -2.
        //
        // Im 8-Bit-unsigned-System:
        // 3 - 5 = 254 (Wraparound)
        //
        // Der finale Borrow muss anzeigen,
        // dass das mathematische Ergebnis negativ war.
        let result = BitSubtractor8::subtract(3, 5);

        assert_eq!(result.difference.value(), 254);
        assert!(result.borrow);
    }

    #[test]
    fn subtract_zero_from_zero() {
        // Sonderfall am unteren Rand.
        let result = BitSubtractor8::subtract(0, 0);

        assert_eq!(result.difference.value(), 0);
        assert!(!result.borrow);
    }
}

