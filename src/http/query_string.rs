use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

// a - 1
// b - 2
// c - None
// d - [None,7,abc]
// e - ==

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // allocating in heap
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}