use crate::mem::register::{ RegisterName, Register8Name, Register16Name, Indirect, FlagName };

use std::fmt::Debug;

pub type Address = u8;
pub type Immediate8 = u8;
pub type Immediate16 = u16;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlagChange {
    UNCH,
    SET,
    RESET,
    DEFER,
}

type FlagStateChange = fn(FlagName) -> FlagChange;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PartialAddressChunk {
    Immediate8(Immediate8),
    Register8Name(Register8Name),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register8PlusFlag {
    r: Register8Name,
    flag: FlagName,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register16PlusImmediate {
    r: Register16Name,
    imm: Immediate16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IndirectPlusFlag {
    i: Indirect,
    flag: FlagName,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Immediate8PlusFlag {
    imm: Immediate8,
    flag: FlagName,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PartialAddress {
    msb: PartialAddressChunk,
    lsb: PartialAddressChunk,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Writable8 {
    Register8Name(Register8Name),
    Address(Address),
    Indirect(Indirect),
    PartialAddress(PartialAddress),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Writable16 {
    Register16Name(Register16Name),
    Address(Address),
    Indirect(Indirect),
    PartialAddress(PartialAddress),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Readable8 {
    Register8Name(Register8Name),
    Register8PlusFlag(Register8PlusFlag),
    Address(Address),
    PartialAddress(PartialAddress),
    Immediate8(Immediate8),
    Immediate8PlusFlag(Immediate8PlusFlag),
    Indirect(Indirect),
    IndirectPlusFlag(IndirectPlusFlag),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Readable16 {
    Register16Name(Register16Name),
    Register16PlusImmediate(Register16PlusImmediate),
    Address(Address),
    PartialAddress(PartialAddress),
    Immediate16(Immediate16),
    Indirect(Indirect),
    IndirectPlusFlag(IndirectPlusFlag),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Readable {
    Readable8(Readable8),
    Readable16(Readable16),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Writable {
    Writable8(Writable8),
    Writable16(Writable16),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShiftDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShiftType {
    Logical,
    Arithmetic,
    Rotate,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    NoOP,
    Load8(Writable8, Readable8),
    Load16(Writable16, Readable16),
    Push(Register16Name),
    Pop(Register16Name),
    Add8(Readable8, Readable8, FlagStateChange),
    AddHL(Register16Name, FlagStateChange),
    AddSP(Immediate8, FlagStateChange),
    Sub8(Readable8, Readable8, FlagStateChange),
    And8(Readable8, FlagStateChange),
    Or8(Readable8, FlagStateChange),
    Xor8(Readable8, FlagStateChange),
    Compare8(Readable8, FlagStateChange),
    Increment8(Writable8, FlagStateChange),
    Increment16(Register16Name),
    Decrement8(Writable8, FlagStateChange),
    Decrement16(Register16Name),
    RegisterShift(RegisterName, ShiftDirection, ShiftType, u8),
}
