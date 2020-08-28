#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register8Name {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register16Name {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RegisterName {
    Register8Name(Register8Name),
    Register16Name(Register16Name),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Indirect {
    HL,
    HLI,
    HLD,
    BC,
    DE,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlagName {
    S,
    Z,
    F5,
    H,
    F3,
    PV,
    N,
    C,
}
