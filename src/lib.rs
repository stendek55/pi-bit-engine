#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitIndexError {
    pub index: u8,
}

impl BitIndexError {
    pub fn new(index: u8) -> Self {
        Self { index }
    }
}

impl std::fmt::Display for BitIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bit index {} is out of range (valid: 0..=7)", self.index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitByte(pub u8);

impl BitByte {
    // --- Konstruktor ---
    pub fn new(val: u8) -> Self {
        BitByte(val)
    }

    // Gibt den inneren u8-Wert zurück
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Führt ein bitweises AND (Und) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist nur dann 1, wenn es in BEIDEN Ausgangszahlen 1 war.
    pub fn bitwise_and(&self, other: BitByte) -> Self {
        BitByte(self.0 & other.0)
    }

    /// Führt ein bitweises OR (Oder) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist 1, wenn es in MINDESTENS EINER der beiden Zahlen 1 war.
    pub fn bitwise_or(&self, other: BitByte) -> Self {
        BitByte(self.0 | other.0)
    }

    /// Führt ein bitweises XOR (Exklusiv-Oder) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist 1, wenn die Bits UNTERSCHIEDLICH sind (eins ist 1, das andere 0).
    pub fn bitwise_xor(&self, other: BitByte) -> Self {
        BitByte(self.0 ^ other.0)
    }

    /// Invertiert alle Bits des aktuellen Byte (bitweises NOT).
    /// Aus jeder 1 wird eine 0, aus jeder 0 eine 1.
    pub fn bitwise_not(&self) -> Self {
        BitByte(!self.0)
    }

    /// Schiebt alle Bits um X Positionen nach links.
    /// Rechts wird mit Nullen aufgefüllt. Bits, die links herausfallen, gehen verloren.
    /// Bei einer Verschiebung >= 8 werden alle Bits entfernt.
    pub fn shift_left(&self, positions: u8) -> Self {
        if positions >= 8 {
            BitByte(0)
        } else {
            BitByte(self.0 << positions)
        }
    }

    /// Schiebt alle Bits um X Positionen nach rechts.
    /// Links wird mit Nullen aufgefüllt. Bits, die rechts herausfallen, gehen verloren.
    /// Bei einer Verschiebung >= 8 werden alle Bits entfernt.
    pub fn shift_right(&self, positions: u8) -> Self {
        if positions >= 8 {
            BitByte(0)
        } else {
            BitByte(self.0 >> positions)
        }
    }

    /// Setzt das Bit am angegebenen Index (0-7) garantiert auf 1.
    pub fn set_bit(mut self, index: u8) -> Result<Self, BitIndexError> {
        if index >= 8 {
            return Err(BitIndexError::new(index));
        }
        // 1. '1 << index' erstellt eine Schablone, bei der NUR das Bit am Index eine 1 ist.
        // 2. '|=' (OR) erzwingt an dieser Stelle eine 1, lässt alle anderen Bits unverändert.
        self.0 |= 1 << index;
        // Wir geben das modifizierte Objekt für das Method Chaining zurück.
        Ok(self)
    }

    /// Löscht das Bit am angegebenen Index (0-7) garantiert (setzt es auf 0).
    pub fn clear_bit(mut self, index: u8) -> Result<Self, BitIndexError> {
        if index >= 8 {
            return Err(BitIndexError::new(index));
        }
        // 1. '1 << index' erstellt die Maske (z.B. 0b0000_0100 bei Index 2).
        // 2. '!' invertiert die Maske (wird zu 0b1111_1011). Überall 1, außer am Ziel-Index.
        // 3. '&=' (AND) behält alle alten Bits bei (da & 1 nix ändert), löscht aber das Ziel-Bit (da & 0 = 0).
        self.0 &= !(1 << index);
        Ok(self)
    }

    /// Invertiert (toggelt) das Bit am angegebenen Index (0-7).
    /// Aus 1 wird 0, aus 0 wird 1.
    pub fn toggle_bit(mut self, index: u8) -> Result<Self, BitIndexError> {
        if index >= 8 {
            return Err(BitIndexError::new(index));
        }
        // 1. '1 << index' erstellt die Maske mit einer einzelnen 1 am Index.
        // 2. '^=' (XOR) dreht den Wert um: 1 ^ 1 wird zu 0, und 0 ^ 1 wird zu 1.
        self.0 ^= 1 << index;
        Ok(self)
    }

    /// Prüft, ob das Bit am angegebenen Index (0-7) den Zustand 1 hat.
    pub fn check_bit(&self, index: u8) -> Result<bool, BitIndexError> {
        if index >= 8 {
            return Err(BitIndexError::new(index));
        }
        // 1. 'self.0 & (1 << index)' isoliert das Bit. Alle anderen Stellen werden zu 0.
        // 2. Wenn das Ergebnis NICHT 0 ist, bedeutet das, dass das Bit am Index eine 1 war.
        // 3. Gibt true zurück wenn gesetzt, andernfalls false.
        Ok((self.0 & (1 << index)) != 0)
    }
}
// ############################################################
// ####################--- TDD Testumgebung ---################
// ############################################################
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        // 0b1100 & 0b1010 = 0b1000
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_and(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1000)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_and(BitByte::new(0x00)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_or() {
        // 0b1100 | 0b1010 = 0b1110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_or(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1110)
        );
        assert_eq!(
            BitByte::new(0x00).bitwise_or(BitByte::new(0x55)),
            BitByte::new(0x55)
        );
    }

    #[test]
    fn test_xor() {
        // 0b1100 ^ 0b1010 = 0b0110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_xor(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_0110)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_xor(BitByte::new(0xFF)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_not() {
        // !0b0000_1111 = 0b1111_0000
        assert_eq!(
            BitByte::new(0b0000_1111).bitwise_not(),
            BitByte::new(0b1111_0000)
        );
        assert_eq!(BitByte::new(0x00).bitwise_not(), BitByte::new(0xFF));
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(
            BitByte::new(0b0000_0001).shift_left(3),
            BitByte::new(0b0000_1000)
        );
        // Überlauf-Verhalten bei u8 (Grenzfall)
        assert_eq!(
            BitByte::new(0b1000_0000).shift_left(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(
            BitByte::new(0b0000_1000).shift_right(3),
            BitByte::new(0b0000_0001)
        );
        assert_eq!(
            BitByte::new(0b0000_0001).shift_right(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_set_bit() {
        // Setze Bit an Index 2 (Wert 4)
        assert_eq!(
            BitByte::new(0b0000_0000)
                .set_bit(2)
                .expect("set_bit should succeed"),
            BitByte::new(0b0000_0100)
        );
        // Bleibt 1, wenn es schon 1 war
        assert_eq!(
            BitByte::new(0b0000_0100)
                .set_bit(2)
                .expect("set_bit should succeed"),
            BitByte::new(0b0000_0100)
        );
    }

    #[test]
    fn test_clear_bit() {
        // Lösche Bit an Index 3 (Wert 8)
        assert_eq!(
            BitByte::new(0b0000_1111)
                .clear_bit(3)
                .expect("clear_bit should succeed"),
            BitByte::new(0b0000_0111)
        );
        // Bleibt 0, wenn es schon 0 war
        assert_eq!(
            BitByte::new(0b0000_0111)
                .clear_bit(3)
                .expect("clear_bit should succeed"),
            BitByte::new(0b0000_0111)
        );
    }

    #[test]
    fn test_toggle_bit() {
        // Aus 0 wird 1
        assert_eq!(
            BitByte::new(0b0000_0000)
                .toggle_bit(4)
                .expect("toggle_bit should succeed"),
            BitByte::new(0b0001_0000)
        );
        // Aus 1 wird 0
        assert_eq!(
            BitByte::new(0b0001_0000)
                .toggle_bit(4)
                .expect("toggle_bit should succeed"),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_check_bit() {
        assert!(
            BitByte::new(0b0000_1000)
                .check_bit(3)
                .expect("check_bit should succeed")
        );
        assert!(
            !BitByte::new(0b0000_1000)
                .check_bit(2)
                .expect("check_bit should succeed")
        );
    }

    #[test]
    fn test_method_chaining() {
        // Komplexer Ablauf über mehrere Mutationen hinweg
        let result = BitByte::new(0)
            .set_bit(0)
            .expect("set_bit should succeed") // -> 0b0000_0001
            .set_bit(3)
            .expect("set_bit should succeed") // -> 0b0000_1001
            .toggle_bit(4)
            .expect("toggle_bit should succeed") // -> 0b0001_1001
            .clear_bit(0)
            .expect("clear_bit should succeed"); // -> 0b0001_1000

        assert_eq!(result, BitByte::new(0b0001_1000));
        assert!(result.check_bit(3).expect("check_bit should succeed"));
        assert!(result.check_bit(4).expect("check_bit should succeed"));
        assert!(!result.check_bit(0).expect("check_bit should succeed"));
    }

    #[test]
    fn test_shift_left_out_of_range() {
        assert_eq!(BitByte::new(0xFF).shift_left(8), BitByte::new(0));
        assert_eq!(BitByte::new(0xFF).shift_left(9), BitByte::new(0));
        assert_eq!(BitByte::new(0xFF).shift_left(255), BitByte::new(0));
    }

    #[test]
    fn test_shift_right_out_of_range() {
        assert_eq!(BitByte::new(0xFF).shift_right(8), BitByte::new(0));
        assert_eq!(BitByte::new(0xFF).shift_right(9), BitByte::new(0));
        assert_eq!(BitByte::new(0xFF).shift_right(255), BitByte::new(0));
    }

    #[test]
    fn test_set_bit_invalid_index() {
        assert!(BitByte::new(0).set_bit(8).is_err());
        assert!(BitByte::new(0).set_bit(255).is_err());
    }

    #[test]
    fn test_clear_bit_invalid_index() {
        assert!(BitByte::new(0xFF).clear_bit(8).is_err());
        assert!(BitByte::new(0xFF).clear_bit(255).is_err());
    }

    #[test]
    fn test_toggle_bit_invalid_index() {
        assert!(BitByte::new(0).toggle_bit(8).is_err());
        assert!(BitByte::new(0).toggle_bit(255).is_err());
    }

    #[test]
    fn test_check_bit_invalid_index() {
        assert!(BitByte::new(0).check_bit(8).is_err());
        assert!(BitByte::new(0).check_bit(255).is_err());
    }

    #[test]
    fn test_bit_index_error_display() {
        let error = BitIndexError::new(8);

        assert_eq!(
            error.to_string(),
            "bit index 8 is out of range (valid: 0..=7)"
        );
    }
}
