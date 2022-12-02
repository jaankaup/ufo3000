pub mod bit;

// Bit testing.
#[cfg(test)]

mod tests {
use crate::bit::zero_bit;
    #[test]
    fn zero_bits_test() {
        let result = zero_bit(0b00000010001100000000000010000000, 7);
        assert_eq!(result, 0b00000010001100000000000000000000);

        let result = zero_bit(0b00000010001100000000000010000000, 8);
        assert_eq!(result, 0b00000010001100000000000010000000);
    }
}
