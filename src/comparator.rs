// ============================================================
// Comparator – 8-Bit Vergleich
// ============================================================
#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonResult {
    Less,
    Equal,
    Greater,
}

pub struct BitComparator8;

impl BitComparator8 {
    pub fn compare(a: u8, b: u8) -> ComparisonResult {
        if a < b {
            return ComparisonResult::Less;
        } else if a > b {
            return ComparisonResult::Greater;
        }
        ComparisonResult::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // TEST 1: Prüft einen normalen Vergleich mit kleinerem Ergebnis
    // ============================================================
    #[test]
    fn test_compare_when_a_is_less_than_b() {
        // Arrange:
        // Wir definieren zwei Zahlen, bei denen a eindeutig kleiner ist.
        let a: u8 = 5;
        let b: u8 = 10;

        // Act:
        // Der Comparator soll beide Zahlen miteinander vergleichen.
        let result = BitComparator8::compare(a, b);

        // Assert:
        // Das Ergebnis muss Less sein.
        assert_eq!(result, ComparisonResult::Less);
    }

    // ============================================================
    // TEST 2: Prüft einen normalen Vergleich mit größerem Ergebnis
    // ============================================================
    #[test]
    fn test_compare_when_a_is_greater_than_b() {
        // Arrange:
        // a ist größer als b.
        let a: u8 = 10;
        let b: u8 = 5;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Greater);
    }

    // ============================================================
    // TEST 3: Prüft zwei identische Zahlen
    // ============================================================
    #[test]
    fn test_compare_when_values_are_equal() {
        // Arrange:
        // Beide Zahlen sind identisch.
        let a: u8 = 42;
        let b: u8 = 42;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Equal);
    }

    // ============================================================
    // TEST 4: Prüft den unteren Rand des Wertebereiches (0)
    // ============================================================
    #[test]
    fn test_compare_zero_with_zero() {
        // Arrange:
        // Der kleinste darstellbare u8-Wert wird mit sich selbst verglichen.
        let a: u8 = 0;
        let b: u8 = 0;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        // Null und Null müssen gleich sein.
        assert_eq!(result, ComparisonResult::Equal);
    }

    // ============================================================
    // TEST 5: Prüft den oberen Rand des Wertebereiches (255)
    // ============================================================
    #[test]
    fn test_compare_max_value_with_max_value() {
        // Arrange:
        // u8 kann maximal 255 darstellen.
        let a: u8 = u8::MAX;
        let b: u8 = u8::MAX;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Equal);
    }

    // ============================================================
    // TEST 6: Null ist kleiner als der maximale u8-Wert
    // ============================================================
    #[test]
    fn test_compare_zero_with_max_value() {
        // Arrange:
        let a: u8 = 0;
        let b: u8 = u8::MAX;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Less);
    }

    // ============================================================
    // TEST 7: Der maximale u8-Wert ist größer als Null
    // ============================================================
    #[test]
    fn test_compare_max_value_with_zero() {
        // Arrange:
        let a: u8 = u8::MAX;
        let b: u8 = 0;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Greater);
    }

    // ============================================================
    // TEST 8: Vergleich direkt an einer Grenze
    // ============================================================
    #[test]
    fn test_compare_adjacent_values() {
        // Arrange:
        // Die Zahlen unterscheiden sich nur um eins.
        let a: u8 = 127;
        let b: u8 = 128;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Less);
    }

    // ============================================================
    // TEST 9: Vergleich in umgekehrter Richtung an derselben Grenze
    // ============================================================
    #[test]
    fn test_compare_adjacent_values_reversed() {
        // Arrange:
        let a: u8 = 128;
        let b: u8 = 127;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Greater);
    }

    // ============================================================
    // TEST 10: Prüft eine Differenz über mehrere Bitpositionen
    // ============================================================
    #[test]
    fn test_compare_values_with_multiple_bit_differences() {
        // Arrange:
        // 0b1010_0000 = 160
        // 0b0101_0000 = 80
        let a: u8 = 0b1010_0000;
        let b: u8 = 0b0101_0000;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Greater);
    }

    // ============================================================
    // TEST 11: Prüft einen Vergleich bei unterschiedlichen Bits
    // ============================================================
    #[test]
    fn test_compare_values_with_different_low_bits() {
        // Arrange:
        // Beide Werte besitzen ähnliche höherwertige Bits.
        let a: u8 = 0b1100_0010;
        let b: u8 = 0b1100_0001;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        // Das niedrigere Bit entscheidet hier zugunsten von a.
        assert_eq!(result, ComparisonResult::Greater);
    }

    // ============================================================
    // TEST 12: Prüft einen Vergleich mit unterschiedlichen High Bits
    // ============================================================
    #[test]
    fn test_compare_values_with_different_high_bits() {
        // Arrange:
        // Das höchstwertige Bit ist bei a gesetzt, bei b nicht.
        let a: u8 = 0b1000_0000;
        let b: u8 = 0b0111_1111;

        // Act:
        let result = BitComparator8::compare(a, b);

        // Assert:
        assert_eq!(result, ComparisonResult::Greater);
    }
}
