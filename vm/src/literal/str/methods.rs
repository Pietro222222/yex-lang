use crate::{error::InterpretResult, literal::TryGet, Value, VirtualMachine, gc::GcRef, List};

pub fn split(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {
    let string: String = args[0].get()?;
    let pat: String = args[1].get()?;
    let arr = string.trim().split(&pat).map(|x| {
        Value::Str(GcRef::new(x.to_string()))
    }).filter(|x| <Value as TryGet<String>>::get(x).unwrap()!= "").collect::<List>().rev();
    Ok(Value::List(arr))
}

pub fn contains(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {
    let string: String = args[0].get()?;
    let substr: String = args[1].get()?;
    Ok(Value::Bool(string.contains(&substr)))
}

pub fn replace(_: *mut VirtualMachine, args: Vec<Value>) -> InterpretResult<Value> {
    let string: String = args[0].get()?;
    let from: String = args[1].get()?;
    let to: String = args[2].get()?;

    Ok(Value::Str(GcRef::new(string.replace(&from, &to))))
}
