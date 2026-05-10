use super::path_bytes::{C_ARR_32, PathBytes};
use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::os::unix::ffi::OsStrExt;

const C_SLASH: u8 = '/' as u8;
const C_DOT: u8 = '.' as u8;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Path {
    dirty: bool,
    inner: VecDeque<PathBytes>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            dirty: true,
            inner: VecDeque::with_capacity(32),
        }
    }

    pub fn root() -> Self {
        "/".into()
    }

    pub fn prepend<T>(&mut self, value: T)
    where
        T: Into<PathBytes>,
    {
        self.push_front(value);
    }

    pub fn push_front<T>(&mut self, value: T)
    where
        T: Into<PathBytes>,
    {
        self.dirty = true;
        self.inner.push_front(value.into());
    }

    pub fn append<T>(&mut self, value: T)
    where
        T: Into<PathBytes>,
    {
        self.push_back(value);
    }

    pub fn push_back<T>(&mut self, value: T)
    where
        T: Into<PathBytes>,
    {
        self.dirty = true;
        self.inner.push_back(value.into());
    }
}

impl From<[u8; C_ARR_32]> for Path {
    fn from(value: [u8; C_ARR_32]) -> Self {
        let mut it = Self::new();
        it.append(value);
        it
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
    use crate::path::path_bytes::Owned;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::ffi::{OsStr, OsString};

    #[test]
    fn impl_from_arr_32() {
        let arr: [u8; C_ARR_32] = [0; C_ARR_32];

        let p: Path = arr.into();

        assert_eq!(
            p.inner,
            VecDeque::from_iter([PathBytes::Owned(Owned::Arr(arr))])
        );
    }

    #[test]
    fn impl_from_small_byte_slice() {
        let p: Path = "small".as_bytes().into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Arr({
                let mut a = [0u8; C_ARR_32];
                a[0] = 's' as u8;
                a[1] = 'm' as u8;
                a[2] = 'a' as u8;
                a[3] = 'l' as u8;
                a[4] = 'l' as u8;
                a
            })),
        );
    }

    #[test]
    fn impl_from_large_byte_slice() {
        let p: Path = "this_supposed_filename_is_longer_than_32".as_bytes().into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Buf(Box::from(
                "this_supposed_filename_is_longer_than_32".as_bytes()
            ))),
        );
    }

    #[test]
    fn impl_from_str() {
        let p: Path = "home".into();

        assert_eq!(p.inner[0], PathBytes::Owned(Owned::Str(Box::from("home"))),);
    }

    #[test]
    fn impl_from_string() {
        let p: Path = String::from("home").into();

        assert_eq!(p.inner[0], PathBytes::Owned(Owned::Str(Box::from("home"))),);
    }

    #[test]
    fn impl_from_os_str() {
        let p: Path = OsStr::new("home").into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Buf(Box::from("home".as_bytes()))),
        );
    }

    #[test]
    fn impl_from_os_string() {
        let p: Path = OsString::from("home").into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Buf(Box::from("home".as_bytes()))),
        );
    }

    #[test]
    fn impl_from_std_path() {
        let p: Path = std::path::Path::new("home").into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Buf(Box::from("home".as_bytes()))),
        );
    }

    #[test]
    fn impl_from_std_pathbuf() {
        let p: Path = std::path::PathBuf::from("home").into();

        assert_eq!(
            p.inner[0],
            PathBytes::Owned(Owned::Buf(Box::from("home".as_bytes()))),
        );
    }
}
