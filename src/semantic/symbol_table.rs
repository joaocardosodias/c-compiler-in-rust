use crate::parser::ast::{Program, Type};
use std::collections::HashMap;
pub struct SymbolType {
    pub var_type: Type,
}
pub struct SymbolInfo {
    scopes: Vec<HashMap<String, SymbolInfo>>,
}

impl SymbolInfo {
    pub fn new(program: Program) -> Self {
        Self{
            scopes:vec![HashMap::new()]
        }
    }
}
