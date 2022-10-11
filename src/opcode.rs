#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiple = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9,
    OpNot = 10,

    OpUnKnown = 99,
}

impl From<u8> for OpCode {
    fn from(op: u8) -> Self {
        match op {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiple,
            6 => OpCode::OpDivide,
            7 => OpCode::OpNil,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            _ => OpCode::OpUnKnown,
        }
    }
}


impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpUnKnown => 99,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiple => 5,
            OpCode::OpDivide => 6,
            OpCode::OpNil => 7,
            OpCode::OpTrue => 8,
            OpCode::OpFalse => 9,
            OpCode::OpNot => 10,
        }
    }
}