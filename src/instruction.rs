#[derive(Debug)]
pub enum Instruction {
    Halt,                // 0
    Set(u16, u16),       // 1
    Push(u16),           // 2
    Pop(u16),            // 3
    Eq(u16, u16, u16),   // 4
    Gt(u16, u16, u16),   // 5
    Jmp(u16),            // 6
    Jt(u16, u16),        // 7
    Jf(u16, u16),        // 8
    Add(u16, u16, u16),  // 9
    Mult(u16, u16, u16), // 10
    Mod(u16, u16, u16),  // 11
    And(u16, u16, u16),  // 12
    Or(u16, u16, u16),   // 13
    Not(u16, u16),       // 14
    Rmem(u16, u16),      // 15
    Wmem(u16, u16),      // 16
    Call(u16),           // 17
    Ret,                 // 18
    Out(u16),            // 19
    In(u16),             // 20
    Noop,                // 21
}
