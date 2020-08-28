mod logic;
mod mem;

fn main() {
    let input = vec![0x16, 0x02];
    let ins = logic::decoder::decode_instruction(&mut input.into_iter().peekable());

    println!("{:?}", ins);
}
