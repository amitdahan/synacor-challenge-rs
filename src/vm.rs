use crate::instruction::Instruction;
use nom::multi::many0;
use nom::number::complete::le_u16;
use nom::IResult;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{stdin, Read};

enum InstructionResult {
    Continue,
    Halt,
}

#[derive(Debug)]
pub struct VM {
    ip: u16,
    memory: [u16; 32768],
    registers: [u16; 8],
    stack: Vec<u16>,
    stdin: VecDeque<u8>,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            ip: 0,
            memory: [0; 32768],
            registers: [0; 8],
            stack: Vec::new(),
            stdin: VecDeque::new(),
        }
    }
}

fn parse_u16s(input: &[u8]) -> Vec<u16> {
    let result: IResult<&[u8], Vec<u16>> = many0(le_u16)(input);
    result.unwrap().1
}

impl VM {
    pub fn load_stdin(&mut self, lines: &[&str]) {
        for line in lines {
            self.stdin.extend(line.bytes());
            self.stdin.push_back(b'\n');
        }
    }

    pub fn load_program(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        for (i, word) in parse_u16s(&buffer).iter().enumerate() {
            self.memory[i] = *word;
        }
    }

    fn next_word(&mut self) -> u16 {
        let word = self.memory[self.ip as usize];
        self.ip += 1;
        word
    }

    fn next_instruction(&mut self) -> Instruction {
        use Instruction::*;
        match self.next_word() {
            0 => Halt,
            1 => Set(self.next_word(), self.next_word()),
            2 => Push(self.next_word()),
            3 => Pop(self.next_word()),
            4 => Eq(self.next_word(), self.next_word(), self.next_word()),
            5 => Gt(self.next_word(), self.next_word(), self.next_word()),
            6 => Jmp(self.next_word()),
            7 => Jt(self.next_word(), self.next_word()),
            8 => Jf(self.next_word(), self.next_word()),
            9 => Add(self.next_word(), self.next_word(), self.next_word()),
            10 => Mult(self.next_word(), self.next_word(), self.next_word()),
            11 => Mod(self.next_word(), self.next_word(), self.next_word()),
            12 => And(self.next_word(), self.next_word(), self.next_word()),
            13 => Or(self.next_word(), self.next_word(), self.next_word()),
            14 => Not(self.next_word(), self.next_word()),
            15 => Rmem(self.next_word(), self.next_word()),
            16 => Wmem(self.next_word(), self.next_word()),
            17 => Call(self.next_word()),
            18 => Ret,
            19 => Out(self.next_word()),
            20 => In(self.next_word()),
            21 => Noop,
            op => panic!("Unknown opcode: {}", op),
        }
    }

    fn resolve_value(&self, value: &u16) -> u16 {
        if (*value > 32767) && (*value < 32776) {
            self.registers[(*value - 32768) as usize]
        } else {
            *value
        }
    }

    fn resolve_register(&self, value: &u16) -> usize {
        if (*value > 32767) && (*value < 32776) {
            (*value - 32768) as usize
        } else {
            panic!("Expected register, got {}", value)
        }
    }

    fn run_current_instruction(&mut self) -> InstructionResult {
        use Instruction::*;
        match self.next_instruction() {
            Halt => return InstructionResult::Halt,
            Out(a) => print!("{}", self.resolve_value(&a) as u8 as char),
            Jmp(a) => self.ip = self.resolve_value(&a),

            Jf(a, b) => {
                if self.resolve_value(&a) == 0 {
                    self.ip = self.resolve_value(&b);
                }
            }
            Jt(a, b) => {
                if self.resolve_value(&a) != 0 {
                    self.ip = self.resolve_value(&b);
                }
            }
            Set(a, b) => self.registers[self.resolve_register(&a)] = self.resolve_value(&b),
            Add(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    (self.resolve_value(&b) + self.resolve_value(&c)) % 32768
            }
            Eq(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    (self.resolve_value(&b) == self.resolve_value(&c)) as u16
            }
            Push(a) => self.stack.push(self.resolve_value(&a)),
            Pop(a) => {
                if let Some(value) = self.stack.pop() {
                    self.registers[self.resolve_register(&a)] = value;
                } else {
                    panic!("Attempted to pop from empty stack");
                }
            }
            Gt(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    (self.resolve_value(&b) > self.resolve_value(&c)) as u16
            }
            And(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    self.resolve_value(&b) & self.resolve_value(&c)
            }
            Or(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    self.resolve_value(&b) | self.resolve_value(&c)
            }
            Not(a, b) => {
                self.registers[self.resolve_register(&a)] = !self.resolve_value(&b) & 0x7FFF
            }
            Call(a) => {
                self.stack.push(self.ip);
                self.ip = self.resolve_value(&a);
            }
            Mult(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    ((self.resolve_value(&b) as u32 * self.resolve_value(&c) as u32) % 32768) as u16
            }
            Mod(a, b, c) => {
                self.registers[self.resolve_register(&a)] =
                    self.resolve_value(&b) % self.resolve_value(&c)
            }
            Rmem(a, b) => {
                self.registers[self.resolve_register(&a)] =
                    self.memory[self.resolve_value(&b) as usize]
            }
            Wmem(a, b) => self.memory[self.resolve_value(&a) as usize] = self.resolve_value(&b),
            Ret => {
                if let Some(address) = self.stack.pop() {
                    self.ip = address;
                } else {
                    return InstructionResult::Halt;
                }
            }
            In(a) => {
                if self.stdin.is_empty() {
                    let mut buf = String::new();
                    stdin().read_line(&mut buf).unwrap();
                    self.stdin.extend(buf.bytes());
                }
                if let Some(value) = self.stdin.pop_front() {
                    self.registers[self.resolve_register(&a)] = value as u16;
                } else {
                    panic!("Attempted to read from empty stdin");
                }
            }
            Noop => {}
        };
        InstructionResult::Continue
    }

    pub fn run_to_completion(&mut self) {
        while let InstructionResult::Continue = self.run_current_instruction() {}
    }
}
