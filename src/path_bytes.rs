use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use crate::arr::arr::Arr;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PathBytes {
    Owned(Owned),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Owned {
    Arr(Arr),
    Buf(Box<[u8]>),
    Str(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Ref {}

impl Into<PathBytes> for &[u8] {
    fn into(self) -> PathBytes {
        PathBytes::Owned(Owned::Buf(Box::from(self)))
    }
}

impl Into<PathBytes> for &str {
    fn into(self) -> PathBytes {
        PathBytes::Owned(Owned::Str(Box::from(self)))
    }
}

impl Into<PathBytes> for String {
    fn into(self) -> PathBytes {
        self.as_str().into()
    }
}

impl Into<PathBytes> for &OsStr {
    fn into(self) -> PathBytes {
        self.as_bytes().into()
    }
}

impl Into<PathBytes> for OsString {
    fn into(self) -> PathBytes {
        self.as_bytes().into()
    }
}

impl Into<PathBytes> for &std::path::Path {
    fn into(self) -> PathBytes {
        self.as_os_str().into()
    }
}

impl Into<PathBytes> for &std::path::PathBuf {
    fn into(self) -> PathBytes {
        self.as_path().into()
    }
}
