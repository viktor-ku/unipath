#![allow(dead_code)]

use std::mem;
use std::str::Utf8Error;

use crate::token_const;

#[derive(Debug)]
pub enum RenderError {
    Utf8Error(Utf8Error),
}

impl From<Utf8Error> for RenderError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::Utf8Error(e) => {
                write!(f, "{}", e)?;
            }
        };

        Ok(())
    }
}

impl std::error::Error for RenderError {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// `0`
    Null,

    /// `.`
    Dot1,

    /// `..`
    Dot2,

    /// `home`
    Home,

    /// `bin`
    Bin,

    /// `etc`
    Etc,

    /// `usr`
    Usr,

    /// `share`
    Share,

    /// `local`
    Local,

    /// `state`
    State,

    /// `.config`
    DotConfig,

    /// .cache
    DotCache,

    /// .local
    DotLocal,

    // unused markers
    UnusedStart,
    UnusedEnd = 253,

    Inline8 = 254,
    Inline16 = 255,
}

impl Token {
    #[inline]
    pub const fn byte(self) -> u8 {
        self as u8
    }

    const fn repr_len(&self) -> usize {
        match self {
            Token::Null => token_const::B_NULL.len(),
            Token::Dot1 => token_const::B_DOT1.len(),
            Token::Dot2 => token_const::B_DOT2.len(),
            Token::Home => token_const::B_HOME.len(),
            Token::Bin => token_const::B_BIN.len(),
            Token::Etc => token_const::B_ETC.len(),
            Token::Usr => token_const::B_USR.len(),
            Token::Share => token_const::B_SHARE.len(),
            Token::Local => token_const::B_LOCAL.len(),
            Token::State => token_const::B_STATE.len(),
            Token::DotConfig => token_const::B_DOT_CONFIG.len(),
            Token::DotCache => token_const::B_DOT_CACHE.len(),
            Token::DotLocal => token_const::B_DOT_LOCAL.len(),
            Token::UnusedStart => 0,
            Token::UnusedEnd => 0,
            Token::Inline8 => 0,
            Token::Inline16 => 0,
        }
    }

    const fn as_repr_bytes(&self) -> &'static [u8] {
        match self {
            Token::Null => &token_const::B_NULL,
            Token::Dot1 => &token_const::B_DOT1,
            Token::Dot2 => &token_const::B_DOT2,
            Token::Home => &token_const::B_HOME,
            Token::Bin => &token_const::B_BIN,
            Token::Etc => &token_const::B_ETC,
            Token::Usr => &token_const::B_USR,
            Token::Share => &token_const::B_SHARE,
            Token::Local => &token_const::B_LOCAL,
            Token::State => &token_const::B_STATE,
            Token::DotConfig => &token_const::B_DOT_CONFIG,
            Token::DotCache => &token_const::B_DOT_CACHE,
            Token::DotLocal => &token_const::B_DOT_LOCAL,
            Token::UnusedStart => &[],
            Token::UnusedEnd => &[],
            Token::Inline8 => &[],
            Token::Inline16 => &[],
        }
    }

    pub fn copy_repr_bytes(&self, dst: &mut [u8]) -> Option<usize> {
        let len = self.repr_len();

        match self {
            Token::Null
            | Token::UnusedStart
            | Token::UnusedEnd
            | Token::Inline8
            | Token::Inline16 => Some(0),
            _ => {
                //
                dst.get_mut(..len).map(|dst| {
                    dst.copy_from_slice(self.as_repr_bytes());
                    len
                })
            }
        }
    }

    fn from_byte<I>(byte: I) -> Option<Self>
    where
        I: Into<u8>,
    {
        let byte = byte.into();
        byte.try_into().ok()
    }

    pub fn render<S>(encoded: S, dst: &mut String) -> Result<usize, RenderError>
    where
        S: AsRef<[u8]>,
    {
        let encoded = encoded.as_ref();

        let written = match Token::try_from(encoded[0]) {
            Ok(token) => {
                match token {
                    Token::Inline8 => {
                        assert!(encoded.len() >= 3);
                        assert!(*encoded.last().unwrap() == Token::Null as u8);

                        let text_bytes = &encoded[2..encoded.len() - 1];
                        let text = std::str::from_utf8(text_bytes)?;
                        dst.push_str(text);
                        text_bytes.len()
                    }
                    _ => 0,
                };

                0
            }
            Err(_) => 0,
        };

        Ok(written)
    }
}

impl From<Token> for u8 {
    #[inline]
    fn from(token: Token) -> Self {
        token as u8
    }
}

impl TryFrom<u8> for Token {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x < Token::UnusedStart as u8 => {
                // SAFETY:
                // - Token is #[repr(u8)].
                // - Null until UnusedStart are contiguous discriminants starting at 0.
                // - We checked x is inside that contiguous valid range.
                Ok(unsafe { mem::transmute::<u8, Token>(x) })
            }
            x if x > Token::UnusedEnd as u8 => {
                // SAFETY:
                // - Token is #[repr(u8)].
                Ok(unsafe { mem::transmute::<u8, Token>(x) })
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn token_is_one_byte() {
        assert_eq!(std::mem::size_of::<Token>(), 1);
    }

    #[test]
    fn a01() {
        let token = Token::Home;

        let buf = &mut [0u8; 32];

        token.copy_repr_bytes(&mut buf[..]);

        assert_eq!(&buf[..token_const::B_HOME.len()], "home".as_bytes());
    }

    #[test]
    fn a02() {
        let token = Token::DotLocal;

        let buf = &mut [0u8; 32];

        token.copy_repr_bytes(&mut buf[4..]);

        assert_eq!(
            &buf[4..4 + token_const::B_DOT_LOCAL.len()],
            ".local".as_bytes()
        );
    }
}
