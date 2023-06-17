use std::ops::{Add, Div, Mul, Sub};
use std::process;

use crate::opcode::*;
use crate::value::Value;

struct Stack {
    data: Vec<Value>,
    pointer: usize,
}

impl Stack {
    const SIZE: usize = 512;

    fn new() -> Self {
        Self {
            data: Vec::new(),
            pointer: 0,
        }
    }

    fn pop(&mut self) -> Value {
        self.pointer += 1;
        match self.data.pop() {
            Some(value) => value,
            None => panic!("stack underflow"),
        }
    }

    fn push(&mut self, value: Value) {
        if self.pointer == Self::SIZE {
            panic!("stack overflow");
        }
        self.data.push(value);
    }
}

pub struct VirtualMachine {
    bytecode: Vec<u8>,
    constants: Vec<Value>,
    pointer: usize,
    stack: Stack,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
            pointer: 0,
            stack: Stack::new(),
        }
    }

    pub fn exec(&mut self, _program: &str) -> Value {
        self.constants.push(Value::from(10.0));
        self.constants.push(Value::from(3.0));

        self.bytecode = vec![OP_CONSTANT, 0x00, OP_CONSTANT, 0x01, OP_MUL, OP_CONSTANT, 0x00, OP_SUB, OP_RETURN];
        self.pointer = 0;

        return self.eval();
    }

    fn eval(&mut self) -> Value {
        loop {
            match self.read_byte() {
                OP_RETURN => return self.stack.pop(),
                OP_CONSTANT => {
                    let index = self.read_byte();
                    let constant = self.constants[index as usize].clone();

                    self.stack.push(constant);
                }
                OP_ADD => self.eval_binary_operation(<Value as Add>::add),
                OP_SUB => self.eval_binary_operation(<Value as Sub>::sub),
                OP_MUL => self.eval_binary_operation(<Value as Mul>::mul),
                OP_DIV => self.eval_binary_operation(<Value as Div>::div),
                opcode => {
                    eprintln!("Unknown opcode: {}", opcode);
                    process::exit(1);
                }
            };
        }
    }

    fn eval_binary_operation(&mut self, operation: fn(Value, Value) -> Value) {
        let right = self.stack.pop();
        let left = self.stack.pop();

        self.stack.push(operation(left, right));
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytecode[self.pointer];
        self.pointer += 1;
        byte
    }
}
