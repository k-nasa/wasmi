use crate::opcode::Opcode;
use crate::types::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Reserved,
    Prefix(VerUintN),
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(VerUintN),
    BrIf(VerUintN),
    BrTable(Vec<VerUintN>, VerUintN),
    Return,
    Call(VerUintN),
    CallIndirect(VerUintN, VerUintN),
    Drop,
    Select,
    GetLocal(VerUintN),
    SetLocal(VerUintN),
    TeeLocal(VerUintN),
    GetGlobal(VerUintN),
    SetGlobal(VerUintN),
    I32Load(u32, u32),
    I64Load(u32, u32),
    F32Load(u32, u32),
    F64Load(u32, u32),
    I32Load8S(u32, u32),
    I32Load8U(u32, u32),
    I32Load16S(u32, u32),
    I32Load16U(u32, u32),
    I64Load8S(u32, u32),
    I64Load8U(u32, u32),
    I64Load16S(u32, u32),
    I64Load16U(u32, u32),
    I64Load32S(u32, u32),
    I64Load32U(u32, u32),
    I32Store(u32, u32),
    I64Store(u32, u32),
    F32Store(u32, u32),
    F64Store(u32, u32),
    I32Store8(u32, u32),
    I32Store16(u32, u32),
    I64Store8(u32, u32),
    I64Store16(u32, u32),
    I64Store32(u32, u32),
    CurrentMemory(VerUintN),
    GrowMemory(VerUintN),
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,
    I64ExtendSI32,
    I64ExtendUI32,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,
    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F32DemoteF64,
    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

impl From<Opcode> for Instruction {
    fn from(opcode: Opcode) -> Self {
        use Instruction::*;
        match opcode {
            Opcode::Unreachable => Unreachable,
            Opcode::Nop => Nop,
            Opcode::Else => Else,
            Opcode::End => End,
            Opcode::Return => Return,
            Opcode::Drop => Drop,
            Opcode::Select => Select,
            Opcode::I32Eqz => I32Eqz,
            Opcode::I32Eq => I32Eq,
            Opcode::I32Ne => I32Ne,
            Opcode::I32LtS => I32LtS,
            Opcode::I32LtU => I32LtU,
            Opcode::I32GtS => I32GtS,
            Opcode::I32GtU => I32GtU,
            Opcode::I32LeS => I32LeS,
            Opcode::I32LeU => I32LeU,
            Opcode::I32GeS => I32GeS,
            Opcode::I32GeU => I32GeU,
            Opcode::I64Eqz => I64Eqz,
            Opcode::I64Eq => I64Eq,
            Opcode::I64Ne => I64Ne,
            Opcode::I64LtS => I64LtS,
            Opcode::I64LtU => I64LtU,
            Opcode::I64GtS => I64GtS,
            Opcode::I64GtU => I64GtU,
            Opcode::I64LeS => I64LeS,
            Opcode::I64LeU => I64LeU,
            Opcode::I64GeS => I64GeS,
            Opcode::I64GeU => I64GeU,
            Opcode::F32Eq => F32Eq,
            Opcode::F32Ne => F32Ne,
            Opcode::F32Lt => F32Lt,
            Opcode::F32Gt => F32Gt,
            Opcode::F32Le => F32Le,
            Opcode::F32Ge => F32Ge,
            Opcode::F64Eq => F64Eq,
            Opcode::F64Ne => F64Ne,
            Opcode::F64Lt => F64Lt,
            Opcode::F64Gt => F64Gt,
            Opcode::F64Le => F64Le,
            Opcode::F64Ge => F64Ge,
            Opcode::I32Clz => I32Clz,
            Opcode::I32Ctz => I32Ctz,
            Opcode::I32Popcnt => I32Popcnt,
            Opcode::I32Add => I32Add,
            Opcode::I32Sub => I32Sub,
            Opcode::I32Mul => I32Mul,
            Opcode::I32DivS => I32DivS,
            Opcode::I32DivU => I32DivU,
            Opcode::I32RemS => I32RemS,
            Opcode::I32RemU => I32RemU,
            Opcode::I32And => I32And,
            Opcode::I32Or => I32Or,
            Opcode::I32Xor => I32Xor,
            Opcode::I32Shl => I32Shl,
            Opcode::I32ShrS => I32ShrS,
            Opcode::I32ShrU => I32ShrU,
            Opcode::I32Rotl => I32Rotl,
            Opcode::I32Rotr => I32Rotr,
            Opcode::I64Clz => I64Clz,
            Opcode::I64Ctz => I64Ctz,
            Opcode::I64Popcnt => I64Popcnt,
            Opcode::I64Add => I64Add,
            Opcode::I64Sub => I64Sub,
            Opcode::I64Mul => I64Mul,
            Opcode::I64DivS => I64DivS,
            Opcode::I64DivU => I64DivU,
            Opcode::I64RemS => I64RemS,
            Opcode::I64RemU => I64RemU,
            Opcode::I64And => I64And,
            Opcode::I64Or => I64Or,
            Opcode::I64Xor => I64Xor,
            Opcode::I64Shl => I64Shl,
            Opcode::I64ShrS => I64ShrS,
            Opcode::I64ShrU => I64ShrU,
            Opcode::I64Rotl => I64Rotl,
            Opcode::I64Rotr => I64Rotr,
            Opcode::F32Abs => F32Abs,
            Opcode::F32Neg => F32Neg,
            Opcode::F32Ceil => F32Ceil,
            Opcode::F32Floor => F32Floor,
            Opcode::F32Trunc => F32Trunc,
            Opcode::F32Nearest => F32Nearest,
            Opcode::F32Sqrt => F32Sqrt,
            Opcode::F32Add => F32Add,
            Opcode::F32Sub => F32Sub,
            Opcode::F32Mul => F32Mul,
            Opcode::F32Div => F32Div,
            Opcode::F32Min => F32Min,
            Opcode::F32Max => F32Max,
            Opcode::F32Copysign => F32Copysign,
            Opcode::F64Abs => F64Abs,
            Opcode::F64Neg => F64Neg,
            Opcode::F64Ceil => F64Ceil,
            Opcode::F64Floor => F64Floor,
            Opcode::F64Trunc => F64Trunc,
            Opcode::F64Nearest => F64Nearest,
            Opcode::F64Sqrt => F64Sqrt,
            Opcode::F64Add => F64Add,
            Opcode::F64Sub => F64Sub,
            Opcode::F64Mul => F64Mul,
            Opcode::F64Div => F64Div,
            Opcode::F64Min => F64Min,
            Opcode::F64Max => F64Max,
            Opcode::F64Copysign => F64Copysign,
            Opcode::I32WrapI64 => I32WrapI64,
            Opcode::I32TruncSF32 => I32TruncSF32,
            Opcode::I32TruncUF32 => I32TruncUF32,
            Opcode::I32TruncSF64 => I32TruncSF64,
            Opcode::I32TruncUF64 => I32TruncUF64,
            Opcode::I64ExtendSI32 => I64ExtendSI32,
            Opcode::I64ExtendUI32 => I64ExtendUI32,
            Opcode::I64TruncSF32 => I64TruncSF32,
            Opcode::I64TruncUF32 => I64TruncUF32,
            Opcode::I64TruncSF64 => I64TruncSF64,
            Opcode::I64TruncUF64 => I64TruncUF64,
            Opcode::F32ConvertSI32 => F32ConvertSI32,
            Opcode::F32ConvertUI32 => F32ConvertUI32,
            Opcode::F32ConvertSI64 => F32ConvertSI64,
            Opcode::F32ConvertUI64 => F32ConvertUI64,
            Opcode::F32DemoteF64 => F32DemoteF64,
            Opcode::F64ConvertSI32 => F64ConvertSI32,
            Opcode::F64ConvertUI32 => F64ConvertUI32,
            Opcode::F64ConvertSI64 => F64ConvertSI64,
            Opcode::F64ConvertUI64 => F64ConvertUI64,
            Opcode::F64PromoteF32 => F64PromoteF32,
            Opcode::I32ReinterpretF32 => I32ReinterpretF32,
            Opcode::I64ReinterpretF64 => I64ReinterpretF64,
            Opcode::F32ReinterpretI32 => F32ReinterpretI32,
            Opcode::F64ReinterpretI64 => F64ReinterpretI64,
            Opcode::Reserved => Reserved,
            _ => todo!("{:x?}", opcode),
        }
    }
}
