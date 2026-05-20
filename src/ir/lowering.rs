use crate::ir::iloc::*;
use crate::parser::ast::*;

pub struct Lowering {
    pub instructions: Vec<IlocInstruction>,
    next_reg: usize,
    next_label: usize,
}
impl Lowering {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            next_reg: 1,
            next_label: 1,
        }
    }
    fn get_new_reg(&mut self) -> usize {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }
    pub fn lower_program(&mut self, program: &Program) -> ProgramIR {
        let mut functions = Vec::new();
        for func in &program.functions {
            functions.push(self.lower_function(func));
        }
        ProgramIR { functions }
    }
    fn lower_function(&mut self, func: &FunctionDecl) -> FunctionIR {
        self.instructions.clear();
        self.next_reg = 1;
        for stmt in &func.body {
            self.lower_stmt(stmt);
        }
        FunctionIR {
            name: func.name.clone(),
            instructions: self.instructions.clone(),
        }
    }
    fn lower_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl {
                name,
                expr,
                var_type,
            } => {
                if let Some(e) = expr {
                    let reg_val = self.lower_expr(e);
                    self.instructions.push(IlocInstruction::Store {
                        src: reg_val,
                        addr: name.clone(),
                    })
                }
            }
            Stmt::Return(expr) => {
                let reg_val = self.lower_expr(expr);
                self.instructions.push(IlocInstruction::Ret {
                    value: Some(reg_val),
                })
            }
            _ => panic!("Fudeu kkkk"),
        }
    }
    fn lower_expr(&mut self, expr: &Expr) -> usize {
        match expr {
            Expr::IntLiteral(val) => {
                let dest = self.get_new_reg();
                self.instructions
                    .push(IlocInstruction::LoadI { imm: *val, dest });
                dest
            }
            Expr::Variable(name) => {
                let dest = self.get_new_reg();
                self.instructions.push(IlocInstruction::Load {
                    addr: name.clone(),
                    dest,
                });
                dest
            }
            Expr::BinOp(op, left, right) => {
                let r_left = self.lower_expr(left);
                let r_right = self.lower_expr(right);
                let dest = self.get_new_reg();
                let instruction = match op {
                    BinaryOp::Add => IlocInstruction::Add {
                        src1: r_left,
                        src2: r_right,
                        dest,
                    },
                    BinaryOp::Divide => IlocInstruction::Div {
                        src1: r_left,
                        src2: r_right,
                        dest,
                    },
                    BinaryOp::Subtract => IlocInstruction::Sub {
                        src1: r_left,
                        src2: r_right,
                        dest,
                    },
                    BinaryOp::Multiply => IlocInstruction::Mul {
                        src1: r_left,
                        src2: r_right,
                        dest,
                    },
                    _ => panic!("fudeu"),
                };
                self.instructions.push(instruction);
                dest
            }
            _ => panic!("Error"),
        }
    }
}
