mod instruction;
mod vm;

use vm::VM;

fn main() {
    let mut vm = VM::default();
    vm.load_program("./vm_challenge/challenge.bin");
    vm.run_to_completion();
}
