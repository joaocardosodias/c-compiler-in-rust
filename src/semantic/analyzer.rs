use crate::parser::ast::{Expr, FunctionDecl, Program, Stmt, Type};
use crate::semantic::symbol_table::{SymbolInfo, SymbolTable};

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    current_return_type: Type,
}
impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            current_return_type: Type::Void,
        }
    }

    pub fn analyzer_program(&mut self, program: &Program) -> Result<(), String> {
        for func in &program.functions {
            self.analyze_function(func)?
        }
        Ok(())
    }
    fn analyze_function(&mut self, func: &FunctionDecl) -> Result<(), String> {
        self.current_return_type = func.function_type;
        self.symbol_table.enter_scope();
        for stmt in &func.body {
            self.analyze_stmt(stmt)?
        }
        self.symbol_table.exit_scope();
        self.current_return_type = Type::Void;
        Ok(())
    }
    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl {
                name,
                expr,
                var_type,
            } => {
                if let Some(expression) = expr {
                    let expr_type = self.analyze_expr(expression)?;
                    if expr_type != *var_type {
                        return Err("Invalid types".to_string());
                    }
                };

                let info = SymbolInfo {
                    var_type: var_type.clone(),
                };
                self.symbol_table.declare(name.clone(), info)?;
                Ok(())
            }

            Stmt::Block(stmts) => {
                self.symbol_table.enter_scope();
                for stmt in stmts {
                    self.analyze_stmt(&stmt)?;
                }
                self.symbol_table.exit_scope();
                Ok(())
            }
            Stmt::Return(expr) => {
                let expr_type = self.analyze_expr(expr)?;
                if self.current_return_type != expr_type {
                    return Err("Invalid types".to_string());
                }
                Ok(())
            }
            Stmt::If {
                condition,
                action,
                else_block,
            } => {
                self.analyze_expr(condition)?;
                self.analyze_stmt(action)?;
                self.analyze_stmt(else_block)?;
                Ok(())
            }
            Stmt::For {
                init,
                condition,
                post,
                body,
            } => {
                self.analyze_stmt(init)?;
                self.analyze_expr(condition)?;
                self.analyze_expr(post)?;
                self.analyze_stmt(body)?;
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.analyze_expr(condition)?;
                self.analyze_stmt(body)?;
                Ok(())
            }
            _ => Err("Invalid commnand".to_string()),
        }
    }
    fn analyze_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::IntLiteral(_) => Ok(Type::Int),
            Expr::Variable(name) => match self.symbol_table.lookup(name) {
                Some(var_type) => Ok(var_type.var_type),
                None => Err(format!("Error:Undeclared variable {}", name)),
            },
            Expr::BinOp(_, left, right) => {
                let type_left = self.analyze_expr(left)?;
                let type_right = self.analyze_expr(right)?;
                if type_left == type_right {
                    Ok(type_left)
                } else {
                    Err("Invalid types".to_string())
                }
            }
            Expr::Call { name, args } => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(Type::Void)
            }
        }
    }
}
