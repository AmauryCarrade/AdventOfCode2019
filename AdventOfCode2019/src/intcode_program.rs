use std::fmt::{Display, Formatter, Write};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Program {
    heap: Vec<usize>,
    pointer: usize,
}

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

#[derive(Debug)]
enum OpCode {
    Arithmetic(Operation),
    Exit,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Program {
    pub fn parse(source_code: &String) -> Result<Program> {
        match source_code
            .split(',')
            .filter(|number_str| !number_str.is_empty())
            .map(|number_str| number_str.parse::<usize>())
            .collect()
        {
            Ok(heap) => Ok(Program { heap, pointer: 0 }),
            Err(_) => Err(Error {
                message: "Invalid source code: invalid numbers.",
            }),
        }
    }

    pub fn patch(&mut self, index: usize, value: usize) {
        self.heap[index] = value;
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.heap.get(index).cloned()
    }

    pub fn output(&self) -> Option<usize> {
        self.get(0)
    }

    fn reset(&mut self) {
        self.pointer = 0;
    }

    pub fn execute(&mut self) -> Result<usize> {
        self.reset();

        loop {
            let forward_result = self.forward()?;
            if !forward_result {
                break Ok(self.output().unwrap());
            }
        }
    }

    fn current(&self) -> Option<usize> {
        self.heap.get(self.pointer).cloned()
    }

    fn after(&self, add: usize) -> Option<usize> {
        self.heap.get(self.pointer + add).cloned()
    }

    fn after_val(&self, add: usize) -> Option<usize> {
        match self.after(add) {
            Some(index) => self.get(index),
            None => None,
        }
    }

    fn compute_operation(&self, operation: Operation, a: usize, b: usize) -> usize {
        match operation {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }

    fn forward(&mut self) -> Result<bool> {
        match self.current() {
            Some(current) => match self.get_opcode(current) {
                Ok(opcode) => match opcode {
                    OpCode::Arithmetic(operation) => match self.after_val(1) {
                        Some(operand1) => match self.after_val(2) {
                            Some(operand2) => match self.after(3) {
                                Some(result_slot) => {
                                    self.heap[result_slot] =
                                        self.compute_operation(operation, operand1, operand2);
                                    self.pointer += 4;
                                    Ok(true)
                                }
                                None => Err(Error {
                                    message: "Broken pointer in operation/result (1|2)",
                                }),
                            },
                            None => Err(Error {
                                message: "Broken pointer in operation/operand2 (1|2)",
                            }),
                        },
                        None => Err(Error {
                            message: "Broken pointer in operation/operand1 (1|2)",
                        }),
                    },
                    OpCode::Exit => Ok(false),
                },
                Err(err) => Err(err),
            },
            None => Err(Error {
                message: "Nothing left and Exit opcode not encountered",
            }),
        }
    }

    fn get_opcode(&self, value: usize) -> Result<OpCode> {
        match value {
            1 => Ok(OpCode::Arithmetic(Operation::Add)),
            2 => Ok(OpCode::Arithmetic(Operation::Multiply)),
            99 => Ok(OpCode::Exit),
            _ => {
                println!("Unexpected opcode: {}", value);
                Err(Error {
                    message: "Unexpected opcode",
                })
            }
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut pointer = 0;
        loop {
            match self.get_opcode(self.heap[pointer]) {
                Ok(opcode) => match opcode {
                    OpCode::Arithmetic(_) => {
                        f.write_fmt(format_args!(
                            "{} {} {} {}",
                            self.heap[pointer],
                            self.heap[pointer + 1],
                            self.heap[pointer + 2],
                            self.heap[pointer + 3]
                        ))?;
                        pointer += 4;
                    }
                    OpCode::Exit => {
                        f.write_fmt(format_args!("{}", self.heap[pointer]))?;
                        pointer += 1;
                    }
                },
                Err(_) => panic!(),
            };

            if pointer >= self.heap.len() {
                break f.write_char('*');
            }

            f.write_char('\n')?;
        }
    }
}
