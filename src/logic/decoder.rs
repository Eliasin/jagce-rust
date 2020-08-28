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

fn decode_unprefixed_instruction<I>(input: &mut Peekable<I>) -> Option<Instruction>
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
