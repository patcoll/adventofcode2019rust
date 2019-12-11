use crate::program;

trait Executable {
    fn execute(&self);
}

impl dyn Executable {
    fn execute(&self) {}
}

#[derive(Debug)]
pub struct Opcode {
    length: u8,
    // execute: dyn FnOnce(&Program),
}

impl Executable for Opcode {
    fn execute(&self) {}
}

// trait Opcode {
//     fn length(&self) -> u8;
// }

// #[derive(Debug)]
// #[derive(Default)]
// pub struct Noop {
//     length: u8,
// }

// impl Opcode for Noop {
//     fn length() -> u8 { 0 }
// }

// #[derive(Debug)]
// #[derive(Default)]
// pub struct Add {
//     length: u8,
//     // fn length() -> u8 { 4 }
// }

// impl Opcode for Add {
//     fn length() -> u8 { 4 }
// }

#[derive(Debug)]
#[derive(Default)]
pub struct Multiply {
}

impl Multiply {
    fn length() -> u8 { 4 }
}

// impl std::marker::Sized for (dyn opcode::Opcode + 'static) {
// }

// pub fn opcodes() -> [dyn Executable; 3] {
//     let opcodes: [dyn Executable; 3] = [
//         Opcode{
//             length: 0,
//             // execute: &fn() -> {  },
//             // execute: |&program| 0,
//         },
//         Opcode{
//             length: 4,
//             // execute: &fn() -> {  },
//             // execute: |&program| 0,
//         },
//         Opcode{
//             length: 4,
//             // execute: &fn() -> {  },
//             // execute: |&program| 0,
//         },
//     ];
//     opcodes
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_opcodes() {
//         let opcodes = opcodes();
//         assert_eq!(opcodes[0].length, 0);
//     }
// }
