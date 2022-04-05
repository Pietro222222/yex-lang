use crate::Value;
pub mod methods;
#[derive(Debug, PartialEq, Clone)]
pub struct Mutable {
    ptr: *mut Value
}

impl Mutable {
    pub fn new(value: Value) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(value)),
        }
    }
    pub fn get(&self) -> Value {
        unsafe {
            &*self.ptr
        }.clone()
    }
    pub fn set(&self, value: Value) {
        unsafe {
            *self.ptr = value;
        }
    }
}


impl std::fmt::Display for Mutable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mutable<{}>", self.get())
    }
}