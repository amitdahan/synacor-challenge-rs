mod instruction;
mod vm;

use vm::VM;

const INPUT: &'static [&'static str] = &[
    // second code
    // third code
    "take tablet",
    // fourth code
    "use tablet",
    "doorway",
    "north",
    "north",
    "bridge",
    "continue",
    "down",
    "east",
    "take empty lantern",
    "west",
    "west",
    "passage",
    "ladder",
    "west",
    "south",
    // fifth code
    "north",
    "take can",
    "west",
    "east",
    "use can",
    "use lantern",
    "east",
    "north",
    "north",
    "ladder",
    "darkness",
    "continue",
    "west",
    "west",
    "west",
    "west",
    "north",
    "take red coin",
    "north",
    "east",
    "take concave coin",
    "down",
    "take corroded coin",
    "up",
    "west",
    "west",
    "take blue coin",
    "up",
    "take shiny coin",
    "down",
    "east",
    "use blue coin",
    "use red coin",
    "use shiny coin",
    "use concave coin",
    "use corroded coin",
    "north",
    "take teleporter",
    "use teleporter",
    // sixth code
    "take business card",
    "take strange book",
];

fn main() {
    let mut vm = VM::default();
    vm.load_program("./vm_challenge/challenge.bin");
    vm.load_stdin(INPUT);
    vm.run_to_completion();
}
