mod instruction;
mod vm;

use vm::VM;

const INPUT: [&str; 2] = ["take tablet", "use tablet"];

fn main() {
    let mut vm = VM::default();
    vm.load_program("./vm_challenge/challenge.bin");
    vm.load_stdin(&INPUT);
    vm.run_to_completion();
}
