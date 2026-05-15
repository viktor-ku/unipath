use std::fmt::Write;
use std::ops::{Range, RangeFrom};

use crate::token::{self, Token};

#[derive(Debug)]
pub struct Path {
    inner: Vec<u8>,
    indicies: Vec<usize>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            indicies: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.indicies.len()
    }

    pub fn display(&self) -> String {
        let mut s = String::with_capacity(self.inner.len() + (self.inner.len() / 50));
        let mut tmp = String::with_capacity(32);

        let mut chunks = self.indicies.chunks_exact(2);

        while let Some(pair) = chunks.next() {
            tmp.clear();

            let encoded = match self.inner.get(pair[0]..pair[1]) {
                Some(it) => it,
                None => {
                    break;
                }
            };

            let _written = Token::render(encoded, &mut tmp).unwrap();

            write!(&mut s, "/{}", tmp).unwrap();
        }

        let last_start_i = match chunks.next() {
            Some(it) => {
                assert!(it.len() == 1);
                it[0]
            }
            None => *self.indicies.last().unwrap(),
        };

        let slice = &self.inner[last_start_i..];

        tmp.clear();

        Token::render(slice, &mut tmp).unwrap();

        write!(&mut s, "/{}", tmp).unwrap();

        s
    }

    pub fn push(&mut self, component: &str) {
        if component.is_empty() {
            return;
        }

        if component.as_bytes()[0] == '/' as u8 {
            return self.push(&component[1..]);
        }

        match component {
            _ => {
                self.indicies.push(self.inner.len());

                self.inner
                    .extend_from_slice(&[Token::Inline8 as u8, component.len() as u8]);
                self.inner.extend_from_slice(component.as_bytes());
                self.inner.extend_from_slice(&[Token::Null as u8]);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn push_with_slashes() {
        let mut path = Path::new();

        path.push("/hello");

        assert_eq!(path.len(), 1);
        assert_eq!(path.display().as_str(), "/hello");

        path.push("/world");

        assert_eq!(path.len(), 2);
        assert_eq!(path.display().as_str(), "/hello/world");
    }

    #[test]
    fn push_several_plain() {
        let mut path = Path::new();

        path.push("hello");

        assert_eq!(path.len(), 1);
        assert_eq!(path.display().as_str(), "/hello");

        path.push("world");

        assert_eq!(path.len(), 2);
        assert_eq!(path.display().as_str(), "/hello/world");
    }
}
