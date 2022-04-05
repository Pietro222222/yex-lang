use crate::{error::InterpretResult, Value, VirtualMachine, literal::TryGet, gc::GcRef};

use super::Mutable;

pub fn init(_: *mut VirtualMachine, _: Vec<Value>) -> InterpretResult<Value> {
    Ok(Value::Mutable(GcRef::new(Mutable::new(Value::Nil))))
}

pub fn set(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {

    let mutable: Mutable = args[0].get()?; 
    mutable.set(args[1].clone());
    Ok(Value::Nil)
}

pub fn get(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {
    let mutable: Mutable = args[0].get()?; 
    Ok(mutable.get())
}
