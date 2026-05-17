use crate::parser::ast::{Expr, FunctionDecl, Program, Stmt};
use crate::semantic::symbol_table::{self, SymbolInfo, SymbolTable};

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
}
impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn analyzer_program(&mut self, program: &Program) -> Result<(), String> {
        for func in &program.functions {
            self.analyze_function(func)?
        }
        Ok(())
    }
    fn analyze_function(&mut self, func: &FunctionDecl) -> Result<(), String> {
        self.symbol_table.enter_scope();
        for stmt in &func.body {
            self.analyze_stmt(stmt)?
        }
        self.symbol_table.exit_scope();
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
                    self.analyze_expr(expression)?
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
                self.analyze_expr(expr)?;
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
        };
        Ok(())
    }
    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::IntLiteral(_) => Ok(()),
            Expr::Variable(name) => match self.symbol_table.lookup(name) {
                Some(_) => Ok(()),
                None => Err(format!("Error:Undeclared variable {}", name)),
            },
            Expr::BinOp(_, left, right) => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
                Ok(())
            }
            Expr::Call { name, args } => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
        }
    }
}
