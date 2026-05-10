use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use crate::arr::arr::Arr;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PathLike {
    Owned(Owned),
}

impl PathLike {
    pub fn inner_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        match self {
            PathLike::Owned(owned) => match owned {
                Owned::Arr(arr) => todo!(),
                Owned::Buf(bytes) => {
                    v.extend_from_slice(&bytes.to_vec());
                }
                Owned::Str(text) => {
                    v.extend_from_slice(text.as_bytes());
                }
            },
        };
        v
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Owned {
    Arr(Arr),
    Buf(Box<[u8]>),
    Str(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Ref {}

impl Into<PathLike> for &[u8] {
    fn into(self) -> PathLike {
        PathLike::Owned(Owned::Buf(Box::from(self)))
    }
}

impl Into<PathLike> for &str {
    fn into(self) -> PathLike {
        PathLike::Owned(Owned::Str(Box::from(self)))
    }
}

impl Into<PathLike> for String {
    fn into(self) -> PathLike {
        self.as_str().into()
    }
}

impl Into<PathLike> for &OsStr {
    fn into(self) -> PathLike {
        self.as_bytes().into()
    }
}

impl Into<PathLike> for OsString {
    fn into(self) -> PathLike {
        self.as_bytes().into()
    }
}

impl Into<PathLike> for &std::path::Path {
    fn into(self) -> PathLike {
        self.as_os_str().into()
    }
}

impl Into<PathLike> for &std::path::PathBuf {
    fn into(self) -> PathLike {
        self.as_path().into()
    }
}
