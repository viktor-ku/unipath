use std::cell::RefCell;
use std::rc::Rc;

use super::Path;

#[derive(Debug, Clone)]
pub struct PathGroup {
    inner: Vec<Item>,
}

#[derive(Debug, Clone)]
enum Item {
    Owned(Path),
    Borrowed(Rc<RefCell<Path>>),
}

impl PathGroup {
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(8),
        }
    }

    pub fn push(&mut self, path: std::borrow::Cow<Path>) {
        todo!()
    }
}
