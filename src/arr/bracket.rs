use super::constants::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bracket {
    _0 = C_0,
    _8 = C_8,
    _16 = C_16,
    _32 = C_32,
}

impl Bracket {
    pub fn ceil(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::_0),
            1..=8 => Some(Self::_8),
            9..=16 => Some(Self::_16),
            17..=32 => Some(Self::_32),
            _ => None,
        }
    }

    #[inline]
    pub const fn zeroed<const T: usize>(&self) -> [u8; T] {
        match self {
            Bracket::_0 => [0u8; T],
            Bracket::_8 => [0u8; T],
            Bracket::_16 => [0u8; T],
            Bracket::_32 => [0u8; T],
        }
    }
}

impl Into<usize> for Bracket {
    fn into(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn nearest_bracket_0_0() {
        assert_eq!(Bracket::ceil(0), Some(Bracket::_0));
    }

    #[test]
    fn nearest_bracket_1_8() {
        assert_eq!(Bracket::ceil(1), Some(Bracket::_8));
    }

    #[test]
    fn nearest_bracket_2_8() {
        assert_eq!(Bracket::ceil(2), Some(Bracket::_8));
    }

    #[test]
    fn nearest_bracket_7_8() {
        assert_eq!(Bracket::ceil(7), Some(Bracket::_8));
    }

    #[test]
    fn nearest_bracket_8_8() {
        assert_eq!(Bracket::ceil(8), Some(Bracket::_8));
    }

    #[test]
    fn nearest_bracket_10_16() {
        assert_eq!(Bracket::ceil(10), Some(Bracket::_16));
    }

    #[test]
    fn nearest_bracket_15_16() {
        assert_eq!(Bracket::ceil(15), Some(Bracket::_16));
    }

    #[test]
    fn nearest_bracket_16_16() {
        assert_eq!(Bracket::ceil(16), Some(Bracket::_16));
    }

    #[test]
    fn nearest_bracket_20_32() {
        assert_eq!(Bracket::ceil(20), Some(Bracket::_32));
    }

    #[test]
    fn nearest_bracket_30_32() {
        assert_eq!(Bracket::ceil(30), Some(Bracket::_32));
    }

    #[test]
    fn nearest_bracket_32_32() {
        assert_eq!(Bracket::ceil(32), Some(Bracket::_32));
    }
}
