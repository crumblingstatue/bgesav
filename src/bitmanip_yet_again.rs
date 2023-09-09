//! I should really make a crate for this
//!

pub trait Bitmanip {
    /// Whether the nth bit is set
    fn nth_bit_set(&self, n: usize) -> bool;
    /// Set the nth bit on or off
    fn set_nth_bit(&mut self, n: usize, on: bool);
    /// Toggle the nth bit
    fn toggle_nth_bit(&mut self, n: usize);
}

impl Bitmanip for u8 {
    fn nth_bit_set(&self, n: usize) -> bool {
        (self & (1 << n)) != 0
    }

    fn set_nth_bit(&mut self, n: usize, on: bool) {
        if on {
            *self |= 1 << n;
        } else {
            *self &= !(1 << n);
        }
    }

    fn toggle_nth_bit(&mut self, n: usize) {
        *self ^= 1 << n;
    }
}

#[test]
fn test_bitmanip() {
    assert!(0b0000_0001_u8.nth_bit_set(0));
    assert!(0b0000_0010_u8.nth_bit_set(1));
    assert!(!0b0000_0010_u8.nth_bit_set(0));
    let mut num = 0;
    num.set_nth_bit(7, true);
    assert_eq!(num, 0b1000_0000);
    num.toggle_nth_bit(7);
    assert_eq!(num, 0);
}
