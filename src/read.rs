use std::{cell::RefCell, rc::Rc};

pub(crate) struct SliceReadView<'a> {
    reader: Rc<RefCell<bt_bencode::read::SliceRead<'a>>>,
}

impl<'a> SliceReadView<'a> {
    pub(crate) fn new(reader: Rc<RefCell<bt_bencode::read::SliceRead<'a>>>) -> Self {
        Self { reader }
    }
}

impl<'a> bt_bencode::read::Read for SliceReadView<'a> {
    #[inline]
    fn next(&mut self) -> Option<Result<u8, bt_bencode::Error>> {
        self.reader.borrow_mut().next()
    }

    #[inline]
    fn peek(&mut self) -> Option<Result<u8, bt_bencode::Error>> {
        self.reader.borrow_mut().peek()
    }

    #[inline]
    fn byte_offset(&self) -> usize {
        self.reader.borrow().byte_offset()
    }
}
