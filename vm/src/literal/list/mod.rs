pub mod methods;

use crate::{
    gc::GcRef,
    literal::{nil, Value},
};

type Link = Option<GcRef<Node>>;
#[derive(Clone, Debug, PartialEq)]
/// Yex lists implementation
pub struct List {
    head: Link,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    elem: Value,
    next: Link,
}

impl List {
    /// Creates a List
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Checks if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head == None
    }

    /// Prepends a value to the end, returning the list
    #[must_use]
    pub fn prepend(&self, elem: Value) -> Self {
        let node = GcRef::new(Node {
            elem,
            next: self.head.clone(),
        });
        Self { head: Some(node) }
    }

    /// Returns the list tail
    #[must_use]
    pub fn tail(&self) -> Self {
        let tail = self.head.as_ref().map(|node| node.next.clone());
        let tail = match tail {
            Some(v) => v,
            None => None,
        };

        Self { head: tail }
    }

    /// Returns the current element
    pub fn head(&self) -> Option<Value> {
        self.head.as_ref().map(|node| node.elem.clone())
    }

    /// Returns a index into the list
    pub fn index(&self, index: usize) -> Value {
        if index == 0 {
            self.head().unwrap_or_else(nil)
        } else {
            let tail = self.tail();
            if tail.is_empty() {
                nil()
            } else {
                tail.index(index - 1)
            }
        }
    }

    /// Returns the list length
    pub fn len(&self) -> usize {
        let mut xs = self.head.as_ref();
        let mut count = 0;
        while xs != None {
            xs = xs.unwrap().next.as_ref();
            count += 1;
        }
        count
    }

    /// Converts list to Vec
    pub fn to_vec(&self) -> Vec<Value> {
        let mut vec = vec![];
        let mut head = self.clone();
        while head.head().is_some() {
            vec.push(head.head().unwrap().to_owned());
            head = head.tail();
        }
        vec
    }

    /// Iterate over all elements of `self`
    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Reverses `self` without consuming it
    #[must_use]
    pub fn rev(&self) -> Self {
        let mut node = self.head.as_ref();
        let mut list = Self::new();
        while let Some(elem) = node {
            list = list.prepend(elem.elem.clone());
            node = elem.next.as_ref()
        }
        list
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (len, value) in self.iter().enumerate() {
            if len != self.len() - 1 {
                write!(f, "{}, ", value)?;
            } else {
                write!(f, "{}", value)?;
            }
        }
        write!(f, "]")
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node.elem.clone()
        })
    }
}

impl FromIterator<Value> for List {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let mut list = Self::new();
        for item in iter.into_iter() {
            list = list.prepend(item)
        }
        list
    }
}
