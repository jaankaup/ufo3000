pub mod bit;

// Bit testing.
#[cfg(test)]

mod tests {
use crate::bit::{zero_bit, one_bit, swap_bit};

    #[test]
    fn zero_bits_test_0() {
        let result = zero_bit(0b00000010001100000000000010000000, 7);
        assert_eq!(result, 0b00000010001100000000000000000000);
    }

    #[test]
    fn zero_bits_test_1() {
        let result = zero_bit(0b00000010001100000000000010000000, 8);
        assert_eq!(result, 0b00000010001100000000000010000000);
    }

    #[test]
    fn one_bits_test_0() {
        let result = one_bit(0b00000010001100000000000010000000, 31);
        assert_eq!(result, 0b10000010001100000000000010000000);
    }

    #[test]
    fn one_bits_test_1() {
        let result = one_bit(0b00000010001100000000000010000001, 0);
        assert_eq!(result, 0b00000010001100000000000010000001);
    }

    #[test]
    fn swap_bits_test_0() {
        let result = swap_bit(0b00000010001100000000000010000000, 2);
        assert_eq!(result, 0b00000010001100000000000010000100);
    }

    #[test]
    fn swap_bits_test_1() {
        let result = swap_bit(0b00000010001100000000000010000000, 7);
        assert_eq!(result, 0b00000010001100000000000000000000);
    }
}
