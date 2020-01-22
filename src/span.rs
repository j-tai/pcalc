use std::rc::Rc;

/// A range of characters in a 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    /// Filename, or None for standard input.
    pub file: Option<Rc<String>>,
    /// Line number, counting from 1.
    pub line: u32,
    /// Column number start, counting from 1.
    pub start: u32,
    /// Column number end, counting from 1.
    pub end: u32,
}
