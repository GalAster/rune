//! Package containing string functions.

use crate::bytes::Bytes;
use crate::context::{ContextError, Module};
use crate::error::Error;

/// into_bytes shim for strings.
fn into_bytes(s: String) -> Bytes {
    Bytes::from_vec(s.into_bytes())
}

fn char_at(s: &str, index: usize) -> Result<Option<char>, Error> {
    if !s.is_char_boundary(index) {
        return Err(Error::msg("index is not a character boundary"));
    }

    Ok(s[index..].chars().next())
}

/// The add operation for strings.
fn add(a: &str, b: &str) -> String {
    let mut string = String::with_capacity(a.len() + b.len());
    string.push_str(a);
    string.push_str(b);
    string
}

/// Get the module for the string package.
pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new(&["std", "string"]);

    module.ty(&["String"]).build::<String>()?;

    module.function(&["String", "from_str"], |s: &str| String::from(s))?;
    module.function(&["String", "new"], String::new)?;
    module.function(&["String", "with_capacity"], String::with_capacity)?;

    module.inst_fn("len", String::len)?;
    module.inst_fn("capacity", String::capacity)?;
    module.inst_fn("clear", String::clear)?;
    module.inst_fn("push", String::push)?;
    module.inst_fn("push_str", String::push_str)?;
    module.inst_fn("reserve", String::reserve)?;
    module.inst_fn("reserve_exact", String::reserve_exact)?;
    module.inst_fn("into_bytes", into_bytes)?;
    module.inst_fn("clone", String::clone)?;
    module.inst_fn("shrink_to_fit", String::shrink_to_fit)?;
    module.inst_fn("char_at", char_at)?;
    module.inst_fn(crate::ADD, add)?;
    module.inst_fn(crate::ADD_ASSIGN, String::push_str)?;
    Ok(module)
}
