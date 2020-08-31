use super::instruction::{ Instruction, Writable8, Readable8, Immediate8 };
use crate::mem::register::{ RegisterName, Register8Name, Register16Name, Indirect };

use std::iter::Peekable;
use std::iter::Iterator;

fn is_prefix_byte(b: u8) -> bool {
    match b {
        0xCB => true,
        _ => false,
    }
}

fn decode_prefixed_instruction<I>(prefix: u8, input: &mut Peekable<I>) -> Option<Instruction>
    where I: Iterator<Item=u8>
{
    None
}

fn construct_load8_from_immediate<I>(r: Register8Name, input: &mut I) -> Option<Instruction>
    where I: Iterator<Item=u8>
{
    let immediate8 = input.next()? as Immediate8;
    Some(Instruction::Load8(Writable8::Register8Name(r), Readable8::Immediate8(immediate8)))
}

fn construct_load8_from_register(dst: Register8Name, src: Register8Name) -> Option<Instruction>
{
    Some(Instruction::Load8(Writable8::Register8Name(dst), Readable8::Register8Name(src)))
}

fn construct_load8_from_indirect(r: Register8Name, indirect: Indirect) -> Option<Instruction>
{
    Some(Instruction::Load8(Writable8::Register8Name(r), Readable8::Indirect(indirect)))
}

fn construct_load8_to_indirect_from_register(indirect: Indirect, register: Register8Name) -> Option<Instruction>
{
    Some(Instruction::Load8(Writable8::Indirect(indirect), Readable8::Register8Name(register)))
}

fn construct_load8_to_indirect_from_immediate<I>(indirect: Indirect, input: &mut I) -> Option<Instruction>
    where I: Iterator<Item=u8>
{
    let immediate8 = input.next()? as Immediate8;
    Some(Instruction::Load8(Writable8::Indirect(indirect), Readable8::Immediate8(immediate8)))
}

fn decode_unprefixed_instruction<I>(input: &mut I) -> Option<Instruction>
    where I: Iterator<Item=u8>
{
    let opcode = input.next()?;
    match opcode {
        0x06 => construct_load8_from_immediate(Register8Name::B, input),
        0x0E => construct_load8_from_immediate(Register8Name::C, input),
        0x16 => construct_load8_from_immediate(Register8Name::D, input),
        0x1E => construct_load8_from_immediate(Register8Name::E, input),
        0x26 => construct_load8_from_immediate(Register8Name::H, input),
        0x2E => construct_load8_from_immediate(Register8Name::L, input),
        0x7F => construct_load8_from_register(Register8Name::A, Register8Name::A),
        0x78 => construct_load8_from_register(Register8Name::A, Register8Name::B),
        0x79 => construct_load8_from_register(Register8Name::A, Register8Name::C),
        0x7A => construct_load8_from_register(Register8Name::A, Register8Name::D),
        0x7B => construct_load8_from_register(Register8Name::A, Register8Name::E),
        0x7C => construct_load8_from_register(Register8Name::A, Register8Name::H),
        0x7D => construct_load8_from_register(Register8Name::A, Register8Name::L),
        0x7E => construct_load8_from_indirect(Register8Name::A, Indirect::HL),
        0x40 => construct_load8_from_register(Register8Name::B, Register8Name::B),
        0x41 => construct_load8_from_register(Register8Name::B, Register8Name::C),
        0x42 => construct_load8_from_register(Register8Name::B, Register8Name::D),
        0x43 => construct_load8_from_register(Register8Name::B, Register8Name::E),
        0x44 => construct_load8_from_register(Register8Name::B, Register8Name::H),
        0x45 => construct_load8_from_register(Register8Name::B, Register8Name::L),
        0x46 => construct_load8_from_indirect(Register8Name::B, Indirect::HL),
        0x48 => construct_load8_from_register(Register8Name::C, Register8Name::B),
        0x49 => construct_load8_from_register(Register8Name::C, Register8Name::C),
        0x4A => construct_load8_from_register(Register8Name::C, Register8Name::D),
        0x4B => construct_load8_from_register(Register8Name::C, Register8Name::E),
        0x4C => construct_load8_from_register(Register8Name::C, Register8Name::H),
        0x4D => construct_load8_from_register(Register8Name::C, Register8Name::L),
        0x4E => construct_load8_from_indirect(Register8Name::C, Indirect::HL),
        0x50 => construct_load8_from_register(Register8Name::D, Register8Name::B),
        0x51 => construct_load8_from_register(Register8Name::D, Register8Name::C),
        0x52 => construct_load8_from_register(Register8Name::D, Register8Name::D),
        0x53 => construct_load8_from_register(Register8Name::D, Register8Name::E),
        0x54 => construct_load8_from_register(Register8Name::D, Register8Name::H),
        0x55 => construct_load8_from_register(Register8Name::D, Register8Name::L),
        0x56 => construct_load8_from_indirect(Register8Name::D, Indirect::HL),
        0x58 => construct_load8_from_register(Register8Name::E, Register8Name::B),
        0x59 => construct_load8_from_register(Register8Name::E, Register8Name::C),
        0x5A => construct_load8_from_register(Register8Name::E, Register8Name::D),
        0x5B => construct_load8_from_register(Register8Name::E, Register8Name::E),
        0x5C => construct_load8_from_register(Register8Name::E, Register8Name::H),
        0x5D => construct_load8_from_register(Register8Name::E, Register8Name::L),
        0x5E => construct_load8_from_indirect(Register8Name::H, Indirect::HL),
        0x60 => construct_load8_from_register(Register8Name::H, Register8Name::B),
        0x61 => construct_load8_from_register(Register8Name::H, Register8Name::C),
        0x62 => construct_load8_from_register(Register8Name::H, Register8Name::D),
        0x63 => construct_load8_from_register(Register8Name::H, Register8Name::E),
        0x64 => construct_load8_from_register(Register8Name::H, Register8Name::H),
        0x65 => construct_load8_from_register(Register8Name::H, Register8Name::L),
        0x66 => construct_load8_from_indirect(Register8Name::H, Indirect::HL),
        0x68 => construct_load8_from_register(Register8Name::L, Register8Name::B),
        0x69 => construct_load8_from_register(Register8Name::L, Register8Name::C),
        0x6A => construct_load8_from_register(Register8Name::L, Register8Name::D),
        0x6B => construct_load8_from_register(Register8Name::L, Register8Name::E),
        0x6C => construct_load8_from_register(Register8Name::L, Register8Name::H),
        0x6D => construct_load8_from_register(Register8Name::L, Register8Name::L),
        0x6E => construct_load8_from_indirect(Register8Name::L, Indirect::HL),
        0x70 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::B),
        0x71 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::C),
        0x72 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::D),
        0x73 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::E),
        0x74 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::H),
        0x75 => construct_load8_to_indirect_from_register(Indirect::HL, Register8Name::L),
        0x36 => construct_load8_to_indirect_from_immediate(Indirect::HL, input),
        _ => None,
    }
}

pub fn decode_instruction<I>(input: &mut Peekable<I>) -> Option<Instruction>
    where I: Iterator<Item=u8>
{
    if is_prefix_byte(*input.peek()?) {
        let prefix = input.next()?;
        decode_prefixed_instruction(prefix, input)
    } else {
        decode_unprefixed_instruction(input)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;

    fn load8_immediate_expected(opcode: u8, immediate: Immediate8) -> Instruction {
        match opcode {
            0x06 => Instruction::Load8(Writable8::Register8Name(Register8Name::B), Readable8::Immediate8(immediate)),
            0x0E => Instruction::Load8(Writable8::Register8Name(Register8Name::C), Readable8::Immediate8(immediate)),
            0x16 => Instruction::Load8(Writable8::Register8Name(Register8Name::D), Readable8::Immediate8(immediate)),
            0x1E => Instruction::Load8(Writable8::Register8Name(Register8Name::E), Readable8::Immediate8(immediate)),
            0x26 => Instruction::Load8(Writable8::Register8Name(Register8Name::H), Readable8::Immediate8(immediate)),
            0x2E => Instruction::Load8(Writable8::Register8Name(Register8Name::L), Readable8::Immediate8(immediate)),
            _ => panic!(),
        }
    }

    #[test]
    fn load8_immediate() {
        const LOAD8_IMMEDIATE_OPCODES: &'static [u8] = &[0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E];
        const LOAD8_IMMEDIATE_IMMEDIATES: &'static [u8] = &[0x06, 0x0E, 0x32, 0xFF, 0x00, 0xAC, 0x99];

        let load8_data = LOAD8_IMMEDIATE_OPCODES.iter().cartesian_product(LOAD8_IMMEDIATE_IMMEDIATES);

        for (opcode, immediate) in load8_data {
            let input: Vec<u8> = vec![*opcode, *immediate];
            let decoded = decode_instruction(&mut input.into_iter().peekable());
            let expected = load8_immediate_expected(*opcode, *immediate);
            assert_eq!(decoded, Some(expected));
        }

    }

}
