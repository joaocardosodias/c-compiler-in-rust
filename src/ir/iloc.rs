#[derive(Debug, Clone, PartialEq)]
pub enum IlocOperand {
    Reg(usize),
    Immediate(i32),
    MemoryAddr(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IlocInstruction {
    LoadI {
        imm: i32,
        dest: usize,
    },
    Load {
        addr: String,
        dest: usize,
    },
    Store {
        src: usize,
        addr: String,
    },
    Add {
        src1: usize,
        src2: usize,
        dest: usize,
    },
    Sub {
        src1: usize,
        src2: usize,
        dest: usize,
    },
    Mul {
        src1: usize,
        src2: usize,
        dest: usize,
    },
    Div {
        src1: usize,
        src2: usize,
        dest: usize,
    },
    CmpLT {
        src1: usize,
        src2: usize,
        dest: usize,
    },

    Jump {
        target: String,
    },
    CJump {
        cond: usize,
        true_label: String,
        false_label: String,
    },

    Call {
        func: String,
        args: Vec<usize>,
        dest: Option<usize>,
    },
    Ret {
        value: Option<usize>,
    },
}
#[derive(Debug, Clone)]
pub struct FunctionIR {
    pub name: String,
    pub instructions: Vec<IlocInstruction>,
}
#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub functions: Vec<FunctionIR>,
}
