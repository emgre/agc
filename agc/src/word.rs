use std::fmt;
use std::ops::*;

/// Trait used by `Word` to specify the number of bits
/// it contains.
///
/// This trait won't be needed once const generics are available
/// in Rust. See https://github.com/rust-lang/rust/issues/44580
pub trait WordSize {
    const NUM_BITS: u8;

    fn mask() -> u16 {
        0xFFFF >> (16 - Self::NUM_BITS) as u16
    }
}

macro_rules! WordSizeX {
    ($id:ident, $size:expr) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $id;
        impl WordSize for $id {
            const NUM_BITS: u8 = $size;
        }
    };
}

WordSizeX!(WS1, 1);
WordSizeX!(WS2, 2);
WordSizeX!(WS3, 3);
WordSizeX!(WS4, 4);
WordSizeX!(WS5, 5);
WordSizeX!(WS6, 6);
WordSizeX!(WS7, 7);
WordSizeX!(WS8, 8);
WordSizeX!(WS9, 9);
WordSizeX!(WS10, 10);
WordSizeX!(WS11, 11);
WordSizeX!(WS12, 12);
WordSizeX!(WS13, 13);
WordSizeX!(WS14, 14);
WordSizeX!(WS15, 15);
WordSizeX!(WS16, 16);

/// Helper type for fixed-size words of 16 bits
/// and less.
///
/// This class makes sure that only the first `WS` bits
/// are set at any time. It also provides all major
/// bitwise operations and helper functions such
/// as `set` and `get`.
///
/// Prefer to use the exported types `W1`, `W2`, ..., `W16`
/// instead of relying on this generic struct.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Word<WS: WordSize> {
    inner: u16,
    _rs: std::marker::PhantomData<WS>,
}

impl<WS: WordSize> Word<WS> {
    pub fn num_bits() -> u8 {
        WS::NUM_BITS
    }

    pub fn mask() -> u16 {
        WS::mask()
    }

    pub fn zero() -> Self {
        Self::from(0)
    }

    pub fn full() -> Self {
        Self::from(WS::mask())
    }

    pub fn as_u16(self) -> u16 {
        self.inner
    }

    pub fn count_ones(&self) -> u8 {
        self.inner.count_ones() as u8
    }

    pub fn count_zeros(&self) -> u8 {
        self.inner.count_zeros() as u8
    }

    pub fn get(&self, index: u8) -> bool {
        debug_assert!(index < Self::num_bits());

        self.inner & (1 << index) != 0
    }

    pub fn set(&mut self, index: u8, value: bool) -> bool {
        debug_assert!(index < Self::num_bits());

        let mask = 1 << index;
        let prev = self.inner & mask;
        self.inner = if value {
            self.inner | mask
        } else {
            self.inner & !mask
        };

        prev != 0
    }
}

impl<WS: WordSize> From<u16> for Word<WS> {
    fn from(from: u16) -> Self {
        Self {
            inner: from & Self::mask(),
            _rs: std::marker::PhantomData,
        }
    }
}

impl<WS: WordSize> Into<u16> for Word<WS> {
    fn into(self) -> u16 {
        self.inner
    }
}

impl<WS: WordSize> Default for Word<WS> {
    fn default() -> Self {
        Self::zero()
    }
}

// Bitwise operators

impl<T: Into<u16>, WS: WordSize> BitOr<T> for Word<WS> {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self::from(self.inner | rhs.into())
    }
}

impl<T: Into<u16>, WS: WordSize> BitOrAssign<T> for Word<WS> {
    fn bitor_assign(&mut self, rhs: T) {
        self.inner = (self.inner | rhs.into()) & Self::mask();
    }
}

impl<T: Into<u16>, WS: WordSize> BitAnd<T> for Word<WS> {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self::from(self.inner & rhs.into())
    }
}

impl<T: Into<u16>, WS: WordSize> BitAndAssign<T> for Word<WS> {
    fn bitand_assign(&mut self, rhs: T) {
        self.inner = self.inner & rhs.into();
    }
}

impl<WS: WordSize> Shl<usize> for Word<WS> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self::from(self.inner << rhs)
    }
}

impl<WS: WordSize> Shr<usize> for Word<WS> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self::from(self.inner >> rhs)
    }
}

// Formatting

impl<WS: WordSize> fmt::Binary for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!("{1:00$b}", WS::NUM_BITS as usize, self.inner);
        f.pad_integral(true, "", &string)
    }
}

impl<WS: WordSize> fmt::Octal for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!(
            "{1:00$o}",
            ((WS::NUM_BITS - 1) / 3 + 1) as usize,
            self.inner
        );
        f.pad_integral(true, "", &string)
    }
}

impl<WS: WordSize> fmt::LowerHex for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!(
            "{1:00$x}",
            ((WS::NUM_BITS - 1) / 4 + 1) as usize,
            self.inner
        );
        f.pad_integral(true, "", &string)
    }
}

impl<WS: WordSize> fmt::UpperHex for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!(
            "{1:00$X}",
            ((WS::NUM_BITS - 1) / 4 + 1) as usize,
            self.inner
        );
        f.pad_integral(true, "", &string)
    }
}

impl<WS: WordSize> fmt::Display for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:o}", self)
    }
}

impl<WS: WordSize> fmt::Debug for Word<WS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// While Rust does not support negative trait bounds, we need
// to implement the `std::convert::From` manually. Otherwise, we get a
// conflicting implementation with `impl<T> std::convert::From<T> for T`.
//
// See https://github.com/rust-lang/rfcs/pull/1148 and
// https://github.com/rust-lang/rust/issues/31844
macro_rules! implement_from {
    (@loop $id:ty, $ws:ty) => {
        // Implement from trait here
        impl From<Word<$id>> for Word<$ws> {
            fn from(from: Word<$id>) -> Word<$ws> {
                Word::from(from.as_u16())
            }
        }
    };
    (@loop $id:ty, $ws:ty, $($tail:tt)*) => {
        implement_from!(@loop $id, $ws);

        // Continue recursion
        implement_from!(@loop $id, $($tail)*);
    };
    ($id:ty: $($ws:ty),*) => {
        implement_from!(@loop $id, $($ws),*);
    }
}

implement_from!(
    WS1: WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS2: WS1,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS3: WS1,
    WS2,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS4: WS1,
    WS2,
    WS3,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS5: WS1,
    WS2,
    WS3,
    WS4,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS6: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS7: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS8: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS9: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS10: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS11: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS12,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS12: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS13,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS13: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS14,
    WS15,
    WS16
);
implement_from!(
    WS14: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS15,
    WS16
);
implement_from!(
    WS15: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS16
);
implement_from!(
    WS16: WS1,
    WS2,
    WS3,
    WS4,
    WS5,
    WS6,
    WS7,
    WS8,
    WS9,
    WS10,
    WS11,
    WS12,
    WS13,
    WS14,
    WS15
);

pub type W1 = Word<WS1>;
pub type W2 = Word<WS2>;
pub type W3 = Word<WS3>;
pub type W4 = Word<WS4>;
pub type W5 = Word<WS5>;
pub type W6 = Word<WS6>;
pub type W7 = Word<WS7>;
pub type W8 = Word<WS8>;
pub type W9 = Word<WS9>;
pub type W10 = Word<WS10>;
pub type W11 = Word<WS11>;
pub type W12 = Word<WS12>;
pub type W13 = Word<WS13>;
pub type W14 = Word<WS14>;
pub type W15 = Word<WS15>;
pub type W16 = Word<WS16>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn word_size_mask() {
        assert_eq!(W1::mask(), 0b0000_0000_0000_0001);
        assert_eq!(W2::mask(), 0b0000_0000_0000_0011);
        assert_eq!(W3::mask(), 0b0000_0000_0000_0111);
        assert_eq!(W4::mask(), 0b0000_0000_0000_1111);
        assert_eq!(W5::mask(), 0b0000_0000_0001_1111);
        assert_eq!(W6::mask(), 0b0000_0000_0011_1111);
        assert_eq!(W7::mask(), 0b0000_0000_0111_1111);
        assert_eq!(W8::mask(), 0b0000_0000_1111_1111);
        assert_eq!(W9::mask(), 0b0000_0001_1111_1111);
        assert_eq!(W10::mask(), 0b0000_0011_1111_1111);
        assert_eq!(W11::mask(), 0b0000_0111_1111_1111);
        assert_eq!(W12::mask(), 0b0000_1111_1111_1111);
        assert_eq!(W13::mask(), 0b0001_1111_1111_1111);
        assert_eq!(W14::mask(), 0b0011_1111_1111_1111);
        assert_eq!(W15::mask(), 0b0111_1111_1111_1111);
        assert_eq!(W16::mask(), 0b1111_1111_1111_1111);
    }

    #[test]
    fn conversion_up() {
        let w1 = W1::from(0b1);
        assert_eq!(W2::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W3::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W4::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W5::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W6::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W7::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W8::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W9::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W10::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W11::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W12::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W13::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W14::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W15::from(w1).as_u16(), w1.as_u16());
        assert_eq!(W16::from(w1).as_u16(), w1.as_u16());
    }

    #[test]
    fn conversion_down() {
        let w16 = W16::from(0b1111_1111_1111_1111);
        assert_eq!(W15::from(w16).as_u16(), 0b111_1111_1111_1111);
        assert_eq!(W14::from(w16).as_u16(), 0b11_1111_1111_1111);
        assert_eq!(W13::from(w16).as_u16(), 0b1_1111_1111_1111);
        assert_eq!(W12::from(w16).as_u16(), 0b1111_1111_1111);
        assert_eq!(W11::from(w16).as_u16(), 0b111_1111_1111);
        assert_eq!(W10::from(w16).as_u16(), 0b11_1111_1111);
        assert_eq!(W9::from(w16).as_u16(), 0b1_1111_1111);
        assert_eq!(W8::from(w16).as_u16(), 0b1111_1111);
        assert_eq!(W7::from(w16).as_u16(), 0b111_1111);
        assert_eq!(W6::from(w16).as_u16(), 0b11_1111);
        assert_eq!(W5::from(w16).as_u16(), 0b1_1111);
        assert_eq!(W4::from(w16).as_u16(), 0b1111);
        assert_eq!(W3::from(w16).as_u16(), 0b111);
        assert_eq!(W2::from(w16).as_u16(), 0b11);
        assert_eq!(W1::from(w16).as_u16(), 0b1);
    }

    #[test]
    fn get_set() {
        let mut w10 = W10::from(0);

        assert_eq!(w10.get(5), false);
        assert_eq!(w10.set(5, true), false);
        assert_eq!(w10.get(5), true);
        assert_eq!(w10.set(5, true), true);
        assert_eq!(w10.as_u16(), 0b00_0010_0000);

        assert!(catch_unwind(|| w10.get(10)).is_err());
        assert!(catch_unwind(move || w10.set(10, true)).is_err());
        assert!(catch_unwind(move || w10.set(10, true)).is_err());
    }

    #[test]
    fn bit_or() {
        let w10 = W10::from(0b00_0110_0110);

        assert_eq!(w10 | W10::zero(), w10);
        assert_eq!(w10 | W10::full(), W10::full());

        assert_eq!(w10 | 0b0000_0011u8, W10::from(0b00_0110_0111)); // with u8
        assert_eq!(w10 | 0b0000_0000_0000_1000u16, W10::from(0b00_0110_1110)); // with u16
        assert_eq!(w10 | W10::from(0b11_1111_0000), W10::from(0b11_1111_0110)); // with same size
        assert_eq!(
            w10 | W16::from(0b1111_0000_0000_1111),
            W10::from(0b00_0110_1111)
        ); // with bigger size
        assert_eq!(w10 | W4::from(0b0111), W10::from(0b00_0110_0111)); // with smaller size
    }

    #[test]
    fn bit_and() {
        let w10 = W10::from(0b00_0110_0110);

        assert_eq!(w10 & W10::zero(), W10::zero());
        assert_eq!(w10 & W10::full(), w10);

        assert_eq!(w10 & 0b0000_0011u8, W10::from(0b00_000_0010));
        assert_eq!(w10 & 0b1111_0000_1111_0000u16, W10::from(0b00_0110_0000));
        assert_eq!(w10 & W10::from(0b11_1111_0000), W10::from(0b00_0110_0000));
        assert_eq!(
            w10 & W16::from(0b1111_0000_0000_1111),
            W10::from(0b00_0000_0110)
        );
        assert_eq!(w10 & W4::from(0b1111), W10::from(0b00_0000_0110));
    }

    #[test]
    fn bit_shift_left() {
        let w10 = W10::from(0b00_0110_0110);

        assert_eq!(w10 << 3, W10::from(0b11_0011_0000));
        assert_eq!(w10 << 0, w10);
        assert_eq!(w10 << 10, W10::zero());
    }

    #[test]
    fn bit_shift_right() {
        let w10 = W10::from(0b00_0110_0110);

        assert_eq!(w10 >> 3, W10::from(0b00_0000_1100));
        assert_eq!(w10 >> 0, w10);
        assert_eq!(w10 >> 10, W10::zero());
    }

    #[test]
    fn formatting_binary() {
        assert_eq!(format!("{:b}", W1::from(0b1)), "1");
        assert_eq!(format!("{:b}", W8::from(0b1010_1010)), "10101010");
        assert_eq!(format!("{:b}", W10::from(0b00_0110_0110)), "0001100110");
        assert_eq!(
            format!("{:^16b}", W10::from(0b00_0110_0110)),
            "   0001100110   "
        );
    }

    #[test]
    fn formatting_octal() {
        assert_eq!(format!("{:o}", W1::from(0b1)), "1");
        assert_eq!(format!("{:o}", W6::from(0b101_010)), "52");
        assert_eq!(format!("{:o}", W10::from(0b0_001_100_110)), "0146");
        assert_eq!(format!("{:^10o}", W10::from(0b0_001_100_110)), "   0146   ");
    }

    #[test]
    fn formatting_lower_hex() {
        assert_eq!(format!("{:x}", W1::from(0b1)), "1");
        assert_eq!(format!("{:x}", W8::from(0b0101_1010)), "5a");
        assert_eq!(format!("{:x}", W10::from(0b00_1111_0110)), "0f6");
        assert_eq!(format!("{:^9x}", W10::from(0b00_1111_0110)), "   0f6   ");
    }

    #[test]
    fn formatting_upper_hex() {
        assert_eq!(format!("{:X}", W1::from(0b1)), "1");
        assert_eq!(format!("{:X}", W8::from(0b0101_1010)), "5A");
        assert_eq!(format!("{:X}", W10::from(0b00_1111_0110)), "0F6");
        assert_eq!(format!("{:^9X}", W10::from(0b00_1111_0110)), "   0F6   ");
    }

    #[test]
    fn display() {
        // Use octal representation
        assert_eq!(format!("{}", W6::from(0b101_010)), "52");
        // Use same display for debug
        assert_eq!(format!("{:?}", W6::from(0b101_010)), "52");
    }
}
