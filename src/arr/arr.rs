use super::bracket::Bracket;
use super::constants::*;
use super::data::Data;
use std::ptr;

#[derive(Debug, thiserror::Error)]
pub enum ArrError {
    #[error("Does not fit into any of the arr brackets")]
    TooLong,

    #[error("Empty")]
    Empty,

    #[error("Impossible")]
    Impossible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Arr {
    data: Data,
}

impl Arr {
    pub fn new() -> Self {
        Self::with_capacity_0()
    }

    pub fn with_capacity_0() -> Self {
        Self {
            data: Data::with_capacity_0(),
        }
    }

    pub fn with_capacity_8() -> Self {
        Self {
            data: Data::with_capacity_8(),
        }
    }

    pub fn with_capacity_16() -> Self {
        Self {
            data: Data::with_capacity_16(),
        }
    }

    pub fn with_capacity_32() -> Self {
        Self {
            data: Data::with_capacity_32(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn as_bytes(&self) -> &[u8] {
        match &self.data {
            Data::_0 => &[],
            Data::_8(it) => &it[..self.len()],
            Data::_16(it) => &it[..self.len()],
            Data::_32(it) => &it[..self.len()],
        }
    }

    pub fn replace_with<S>(&mut self, input: S) -> Result<(), ArrError>
    where
        S: AsRef<[u8]>,
    {
        let input = input.as_ref();
        let input_len = input.len();

        let self_capacity = self.capacity();

        match (self_capacity, input_len) {
            (0, 0) => Ok(()),

            (_, C_OUT_OF_BOUNDS..) => Err(ArrError::TooLong),

            (s, i) if s > i => {
                let src_ptr = input.as_ptr();

                self.clear();

                // SAFETY: cannot be null because
                // self_capacity is never 0 in case it's bigger than input
                let dst_ptr = self.data.as_mut_ptr();

                #[cfg(test)]
                {
                    assert!(!dst_ptr.is_null());
                }

                unsafe {
                    ptr::copy_nonoverlapping(src_ptr, dst_ptr, input_len);
                }

                Ok(())
            }

            (s, i) if s == i => {
                let src_ptr = input.as_ptr();

                // SAFETY: self_capacity is never 0 since
                // we've already covered (0, 0) in another
                // match arm
                let dst_ptr = self.data.as_mut_ptr();

                #[cfg(test)]
                {
                    assert!(!dst_ptr.is_null());
                }

                unsafe {
                    ptr::copy_nonoverlapping(src_ptr, dst_ptr, input_len);
                };

                Ok(())
            }

            (s, i) if s < i => {
                // SAFETY: fails only when i > 32
                // which by this point we've already checked for
                let bracket = Bracket::ceil(i).unwrap();

                self.data = Data::with_capacity(bracket);

                self.replace_with(input)
            }

            _ => Err(ArrError::Impossible),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl TryFrom<&[u8]> for Arr {
    type Error = ArrError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut it = Self::new();
        it.replace_with(value)?;
        Ok(it)
    }
}

impl TryFrom<&str> for Arr {
    type Error = ArrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C_WORD_0: &[u8] = "".as_bytes();
    const C_WORD_8: &[u8] = "12345678".as_bytes();
    const C_WORD_16: &[u8] = "1234567812345678".as_bytes();
    const C_WORD_32: &[u8] = "12345678123456781234567812345678".as_bytes();

    mod various {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn grows() {
            let mut arr = Arr::try_from("home").unwrap();

            assert_eq!(arr.len(), 4);
            assert_eq!(arr.capacity(), 8);
            assert_eq!(arr.as_bytes(), "home".as_bytes());

            arr.replace_with("I have changed my mind").unwrap();

            assert_eq!(arr.len(), 22);
            assert_eq!(arr.capacity(), 32);
            assert_eq!(arr.as_bytes(), "I have changed my mind".as_bytes());
        }

        #[test]
        fn shrinks() {
            let mut arr = Arr::try_from("Imagine a long queue").unwrap();

            assert_eq!(arr.len(), 20);
            assert_eq!(arr.capacity(), 32);
            assert_eq!(arr.as_bytes(), "Imagine a long queue".as_bytes());

            arr.replace_with("to root").unwrap();

            assert_eq!(arr.len(), 7);
            assert_eq!(arr.capacity(), 32);
            assert_eq!(arr.as_bytes(), "to root".as_bytes());
        }
    }

    mod input_n_arr_same {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn both_0() {
            let mut arr = Arr::with_capacity_0();
            arr.replace_with(C_WORD_0).unwrap();

            assert_eq!(arr.len(), 0);
            assert_eq!(arr.capacity(), 0);
            assert_eq!(arr.as_bytes(), C_WORD_0);
        }

        #[test]
        fn both_8() {
            let mut arr = Arr::with_capacity_8();
            arr.replace_with(C_WORD_8).unwrap();

            assert_eq!(arr.len(), 8);
            assert_eq!(arr.capacity(), 8);
            assert_eq!(arr.as_bytes(), C_WORD_8);
        }

        #[test]
        fn both_16() {
            let mut arr = Arr::with_capacity_16();
            arr.replace_with(C_WORD_16).unwrap();

            assert_eq!(arr.len(), 16);
            assert_eq!(arr.capacity(), 16);
            assert_eq!(arr.as_bytes(), C_WORD_16);
        }

        #[test]
        fn both_32() {
            let mut arr = Arr::with_capacity_32();
            arr.replace_with(C_WORD_32).unwrap();

            assert_eq!(arr.len(), 32);
            assert_eq!(arr.capacity(), 32);
            assert_eq!(arr.as_bytes(), C_WORD_32);
        }
    }

    mod input_0_arr_x {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn arr_0() {
            let mut arr = Arr::with_capacity_0();
            arr.replace_with(C_WORD_0).unwrap();

            assert_eq!(arr.len(), 0);
            assert_eq!(arr.capacity(), 0);
            assert_eq!(arr.as_bytes(), C_WORD_0);
        }

        #[test]
        fn arr_8() {
            let mut arr = Arr::with_capacity_8();
            arr.replace_with(C_WORD_0).unwrap();

            assert_eq!(arr.len(), 0);
            assert_eq!(arr.capacity(), 8);
            assert_eq!(arr.as_bytes(), C_WORD_0);
        }

        #[test]
        fn arr_16() {
            let mut arr = Arr::with_capacity_16();
            arr.replace_with(C_WORD_0).unwrap();

            assert_eq!(arr.len(), 0);
            assert_eq!(arr.capacity(), 16);
            assert_eq!(arr.as_bytes(), C_WORD_0);
        }

        #[test]
        fn arr_32() {
            let mut arr = Arr::with_capacity_32();
            arr.replace_with(C_WORD_0).unwrap();

            assert_eq!(arr.len(), 0);
            assert_eq!(arr.capacity(), 32);
            assert_eq!(arr.as_bytes(), C_WORD_0);
        }
    }

    mod input_x_arr_starts_at_0 {
        use super::*;
        use pretty_assertions::assert_eq;

        fn assert_same_size(word: &[u8], size: usize) {
            let mut arr = Arr::new();
            arr.replace_with(word).unwrap();

            assert_eq!(arr.len(), size);
            assert_eq!(arr.capacity(), size);
            assert_eq!(arr.as_bytes(), word);
        }

        #[test]
        fn input_5() {
            let word = "12345".as_bytes();

            let mut arr = Arr::new();
            arr.replace_with(word).unwrap();

            assert_eq!(arr.len(), 5);
            assert_eq!(arr.capacity(), 8);
            assert_eq!(arr.as_bytes(), word);
        }

        #[test]
        fn input_7() {
            let word = "1234567".as_bytes();

            let mut arr = Arr::new();
            arr.replace_with(word).unwrap();

            assert_eq!(arr.len(), 7);
            assert_eq!(arr.capacity(), 8);
            assert_eq!(arr.as_bytes(), word);
        }

        #[test]
        fn input_9() {
            let word = "123456789".as_bytes();

            let mut arr = Arr::new();
            arr.replace_with(word).unwrap();

            assert_eq!(arr.len(), 9);
            assert_eq!(arr.capacity(), 16);
            assert_eq!(arr.as_bytes(), word);
        }

        #[test]
        fn input_0() {
            assert_same_size(C_WORD_0, 0);
        }

        #[test]
        fn input_8() {
            assert_same_size(C_WORD_8, 8);
        }

        #[test]
        fn input_16() {
            assert_same_size(C_WORD_16, 16);
        }

        #[test]
        fn input_32() {
            assert_same_size(C_WORD_32, 32);
        }
    }
}
