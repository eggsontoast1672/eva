mod opcode;
mod value;
mod vm;

use crate::vm::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    let result = vm.exec(
        r"
        42
    ",
    );

    println!("{:?}", result);
    println!("All done!");
}
