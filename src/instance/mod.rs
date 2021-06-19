use crate::module::Module;
use crate::runtime::{error::RuntimeError, Runtime};
use crate::types::*;

#[derive(Debug)]
pub struct Instance {
    module: Module,
}

type ValueStack = Vec<RuntimeValue>;

impl Instance {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn invoke(
        &self,
        name: impl AsRef<str>,
        args: Vec<RuntimeValue>,
    ) -> Result<ValueStack, RuntimeError> {
        let index = self.resolve_function_name(name.as_ref());
        let index = match index {
            None => return Err(RuntimeError::NotFound(name.as_ref().to_string())),
            Some(i) => i,
        };

        let func_type = self.get_func_type(index)?;
        let func = self.get_function(index)?;

        Instance::validate(func_type, &args)?; // argsとfunc_type.paramsの個数、型をチェックする + errorをいい感じに表示してあげたい

        let mut runtime = Runtime::new(func.code.clone());
        let stack = runtime.execute(&args)?;

        Ok(stack)
    }

    fn resolve_function_name(&self, name: impl AsRef<str>) -> Option<usize> {
        let export_section = &self.module.export_section;

        let exports = match export_section {
            None => return None,
            Some(e) => &e.entries,
        };

        let entry = exports.iter().find(|x| x.field_str == name.as_ref());

        entry.map(|x| x.index as usize)
    }

    fn get_function(&self, index: usize) -> Result<&FunctionBody, RuntimeError> {
        let code_section = &self.module.code_section.as_ref();
        if code_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection);
        }
        let function = code_section.unwrap().bodies.get(index).unwrap();

        Ok(function)
    }

    fn get_func_type(&self, index: usize) -> Result<&FuncType, RuntimeError> {
        let func_section = &self.module.function_section.as_ref();
        if func_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection); // fix error type
        }

        let t = func_section.unwrap().types.get(index).unwrap();

        let type_section = &self.module.type_section.as_ref();
        if type_section.is_none() {
            return Err(RuntimeError::ExpectCodeSection); // fix error type
        }

        let types = type_section.unwrap().entries.get(*t as usize).unwrap();

        Ok(types)
    }

    fn validate(func_type: &FuncType, args: &[RuntimeValue]) -> Result<(), RuntimeError> {
        let args_types: Vec<_> = args.iter().map(RuntimeValue::to_type).collect();

        let expect = func_type.params.clone();
        if expect != args_types {
            return Err(RuntimeError::InvalidArgs(expect, args_types));
        }

        Ok(())
    }
}
