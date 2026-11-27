use crate::BitByte;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSubtractor8Result {
    pub difference: BitByte,
    pub borrow: bool,
}

pub struct BitSubtractor8;

// --------------------------------------------------
// 1-Bit Full Subtractor
// --------------------------------------------------

pub fn full_subber(a_bit: bool, b_bit: bool, borrow_in: bool) -> (bool, bool) {
    // Difference berechnen
    let partial_difference = a_bit ^ b_bit;
    let difference_bit = partial_difference ^ borrow_in;
    // Borrow aus A und B
    let borrow_due_to_ab = !a_bit && b_bit;
    // Borrow aus dem eingehenden Borrow
    let borrow_due_to_input = !partial_difference && borrow_in;
    // Beide Borrow-Ursachen zusammenführen
    let borrow_out = borrow_due_to_ab || borrow_due_to_input;

    (difference_bit, borrow_out)
}

// --------------------------------------------------
// 8-Bit Subtractor mit Borrow Chain
// --------------------------------------------------

impl BitSubtractor8 {
    pub fn subtract(a: u8, b: u8) -> BitSubtractor8Result {
        let a_byte = BitByte::new(a);
        let b_byte = BitByte::new(b);

        let mut difference = BitByte::new(0);
        let mut borrow = false;

        // Vom niederwertigsten zum höchstwertigen Bit
        for index in 0..8 {
            let a_bit = a_byte.check_bit(index).expect("index muss valide sein!");
            let b_bit = b_byte.check_bit(index).expect("index muss valide sein!");

            // 1-Bit Subtraktion
            let (difference_bit, borrow_out) = full_subber(a_bit, b_bit, borrow);

            // Ergebnisbit in das Ergebnisbyte übernehmen
            if difference_bit {
                difference = difference.set_bit(index).expect("index muss valide sein!");
            }

            // Borrow an die nächste Bitposition weiterreichen
            borrow = borrow_out;
        }

        BitSubtractor8Result { difference, borrow }
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

