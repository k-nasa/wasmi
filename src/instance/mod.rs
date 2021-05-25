use crate::module::Module;
use crate::types::*;

#[derive(Debug)]
pub struct Instance {
    module: Module,
}

impl Instance {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn invoke(&self, name: impl AsRef<str>) -> Result<Vec<RuntimeValue>, RuntimeError> {
        let index = self.resolve_function_name(name.as_ref());
        let index = match index {
            None => return Err(RuntimeError::NotFound(name.as_ref().to_string())),
            Some(i) => i,
        };

        let func = self.get_function(index)?;
        dbg!(func);
        Ok(vec![])
    }

    fn resolve_function_name(&self, name: impl AsRef<str>) -> Option<usize> {
        let export_section = &self.module.export_section;

        let exports = match export_section {
            None => return None,
            Some(e) => &e.entries,
        };

        let entry = exports.iter().find(|x| &x.field_str == name.as_ref());

        entry.and_then(|x| Some(x.index as usize))
    }

    fn get_function(&self, index: usize) -> Result<&FunctionBody, RuntimeError> {
        let code_section = &self.module.code_section.as_ref();
        if code_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection);
        }
        let function = code_section.unwrap().bodies.get(index).unwrap();

        return Ok(function);
    }
}

use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    ExpectCodeSection,
    IOError(std::io::Error),
}

impl Error for RuntimeError {}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RuntimeError::*;
        match self {
            NotFound(name) => write!(f, "'{}'' is not found", name),
            ExpectCodeSection => {
                write!(f, "not found code section. wasmi is expected code section")
            }
            IOError(i) => write!(f, "io error: {}", i),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
