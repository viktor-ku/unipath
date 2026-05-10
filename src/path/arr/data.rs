use super::bracket::Bracket;
use super::constants::*;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Data {
    _0,
    _8([u8; Bracket::_8 as usize]),
    _16([u8; Bracket::_16 as usize]),
    _32([u8; Bracket::_32 as usize]),
}

impl Data {
    pub fn new() -> Self {
        Self::with_capacity_0()
    }

    pub fn with_capacity(capacity: Bracket) -> Self {
        match capacity {
            Bracket::_0 => Self::_0,
            Bracket::_8 => Self::_8(Bracket::_8.zeroed()),
            Bracket::_16 => Self::_16(Bracket::_16.zeroed()),
            Bracket::_32 => Self::_32(Bracket::_32.zeroed()),
        }
    }

    pub fn with_capacity_0() -> Self {
        Self::with_capacity(Bracket::_0)
    }

    pub fn with_capacity_8() -> Self {
        Self::with_capacity(Bracket::_8)
    }

    pub fn with_capacity_16() -> Self {
        Self::with_capacity(Bracket::_16)
    }

    pub fn with_capacity_32() -> Self {
        Self::with_capacity(Bracket::_32)
    }

    pub fn len(&self) -> usize {
        match self {
            Data::_0 => C_0,
            Data::_8(it) => it.iter().position(|it| *it == C_ZERO).unwrap_or(C_8),
            Data::_16(it) => it.iter().position(|it| *it == C_ZERO).unwrap_or(C_16),
            Data::_32(it) => it.iter().position(|it| *it == C_ZERO).unwrap_or(C_32),
        }
    }

    pub fn capacity(self) -> usize {
        <Data as Into<Bracket>>::into(self) as usize
    }

    pub fn bracket(&self) -> Bracket {
        match self {
            Data::_0 => Bracket::_0,
            Data::_8(_) => Bracket::_8,
            Data::_16(_) => Bracket::_16,
            Data::_32(_) => Bracket::_32,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match self {
            Data::_0 => ptr::null_mut(),
            Data::_8(it) => it.as_mut_ptr(),
            Data::_16(it) => it.as_mut_ptr(),
            Data::_32(it) => it.as_mut_ptr(),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Data::_0 => {}
            Data::_8(it) => {
                unsafe { it.as_mut_ptr().write_bytes(C_ZERO, Bracket::_8 as usize) };
            }
            Data::_16(it) => {
                unsafe { it.as_mut_ptr().write_bytes(C_ZERO, Bracket::_16 as usize) };
            }
            Data::_32(it) => {
                unsafe { it.as_mut_ptr().write_bytes(C_ZERO, Bracket::_32 as usize) };
            }
        }
    }
}

impl Into<Bracket> for Data {
    fn into(self) -> Bracket {
        match self {
            Data::_0 => Bracket::_0,
            Data::_8(_) => Bracket::_8,
            Data::_16(_) => Bracket::_16,
            Data::_32(_) => Bracket::_32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
}
