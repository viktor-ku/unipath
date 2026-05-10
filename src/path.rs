use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::os::unix::ffi::OsStrExt;
use std::path::Display;

use crate::path_like::PathLike;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Path {
    inner: VecDeque<PathLike>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::with_capacity(32),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn inner_bytes(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.inner.len());
        for it in &self.inner {
            v.extend_from_slice(&it.inner_bytes());
        }
        v
    }

    pub fn prepend<T>(&mut self, value: T)
    where
        T: Into<PathLike>,
    {
        self.push_front(value);
    }

    pub fn push_front<T>(&mut self, value: T)
    where
        T: Into<PathLike>,
    {
        self.inner.push_front(value.into());
    }

    pub fn push_back<T>(&mut self, value: T)
    where
        T: Into<PathLike>,
    {
        self.inner.push_back(value.into());
    }

    pub fn append<T>(&mut self, value: T)
    where
        T: Into<PathLike>,
    {
        self.push_back(value);
    }

    pub fn join<T>(&mut self, value: T)
    where
        T: Into<PathLike>,
    {
        self.append(value);
    }
}

impl From<&[u8]> for Path {
    fn from(value: &[u8]) -> Self {
        let mut it = Self::new();
        it.append(value);
        it
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut it = Self::new();
        it.append(value);
        it
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&OsStr> for Path {
    fn from(value: &OsStr) -> Self {
        value.as_bytes().into()
    }
}

impl From<OsString> for Path {
    fn from(value: OsString) -> Self {
        value.as_bytes().into()
    }
}

impl From<&std::path::Path> for Path {
    fn from(value: &std::path::Path) -> Self {
        value.as_os_str().into()
    }
}

impl From<std::path::PathBuf> for Path {
    fn from(value: std::path::PathBuf) -> Self {
        value.as_path().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod push_back {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_literal_home() {
            let mut path = Path::new();
            path.push_back("home");
            assert_eq!(path.inner_bytes(), "home".as_bytes());
        }

        #[test]
        fn one_bunch() {
            let mut path = Path::new();
            path.push_back("a/b/c");
            assert_eq!(path.inner_bytes(), "a/b/c".as_bytes());
        }

        #[test]
        fn one_after_another() {
            let mut path = Path::new();
            path.push_back("one");
            path.push_back("two");
            assert_eq!(path.inner_bytes(), "onetwo".as_bytes());
        }
    }

    mod push_front {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_literal_home() {
            let mut path = Path::new();
            path.push_front("home");
            assert_eq!(path.inner_bytes(), "home".as_bytes());
        }

        #[test]
        fn one_bunch() {
            let mut path = Path::new();
            path.push_front("a/b/c");
            assert_eq!(path.inner_bytes(), "a/b/c".as_bytes());
        }

        #[test]
        fn one_after_another() {
            let mut path = Path::new();
            path.push_front("one");
            path.push_front("two");
            assert_eq!(path.inner_bytes(), "twoone".as_bytes());
        }
    }

    mod path_new {
        use super::*;

        #[test]
        fn creates_new() {
            let path = Path::new();
            assert!(path.is_empty());
        }
    }
}
